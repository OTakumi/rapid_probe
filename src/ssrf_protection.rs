//! SSRF（Server-Side Request Forgery）対策モジュール
//!
//! このモジュールは、悪意のあるURLやIPアドレスへのアクセスをブロックし、
//! SSRF攻撃を防ぐための包括的な検証ロジックを提供します。

use anyhow::{anyhow, Result};
use ipnetwork::{Ipv4Network, Ipv6Network};
use std::net::IpAddr;
use url::Url;

/// URLのバリデーション
///
/// SSRF（Server-Side Request Forgery）攻撃を防ぐため、
/// プライベートIPアドレスやlocalhostへのアクセスを包括的に制限
///
/// # ブロックされるアドレス
///
/// - localhost（localhost, 127.0.0.0/8, ::1）
/// - プライベートIPv4（10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16）
/// - Link-local（169.254.0.0/16, fe80::/10）
/// - プライベートIPv6（fc00::/7）
/// - 特殊アドレス（0.0.0.0/8, ::, 255.255.255.255）
///
/// # Arguments
///
/// * `url` - 検証するURL
///
/// # Returns
///
/// URLが安全な場合は`Ok(())`、危険な場合は`Err`
///
/// # Examples
///
/// ```
/// use url::Url;
/// use rapid_probe::ssrf_protection::validate_url;
///
/// let safe_url = Url::parse("https://example.com").unwrap();
/// assert!(validate_url(&safe_url).is_ok());
///
/// let dangerous_url = Url::parse("http://localhost").unwrap();
/// assert!(validate_url(&dangerous_url).is_err());
/// ```
pub fn validate_url(url: &Url) -> Result<()> {
    // スキームの検証
    let scheme = url.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(anyhow!(
            "invalid URL scheme '{}': only http and https are allowed",
            scheme
        ));
    }

    // ホストの検証
    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("URL must have a host"))?;

    // ホスト名ベースのチェック（ドメイン名がlocalhostの場合）
    if host == "localhost" || host.ends_with(".localhost") {
        return Err(anyhow!(
            "localhost URLs are not allowed for security reasons: {}",
            host
        ));
    }

    // IPv6アドレスの角括弧を除去
    let host_without_brackets = host.trim_start_matches('[').trim_end_matches(']');

    // IPアドレスとしてパース
    if let Ok(ip_addr) = host_without_brackets.parse::<IpAddr>() {
        // IPアドレスベースの包括的なチェック
        validate_ip_address(&ip_addr)?;
    }

    // ドメイン名の場合、基本的なホスト名検証のみ
    // DNS解決によるプライベートIP検出は、パフォーマンスとタイムアウトの問題があるため、
    // ここでは行わず、reqwestのリダイレクト制限に依存する

    Ok(())
}

/// IPアドレスのバリデーション
///
/// SSRF攻撃を防ぐため、危険なIPアドレスを包括的にブロック
///
/// # Arguments
///
/// * `ip` - 検証するIPアドレス
///
/// # Returns
///
/// IPアドレスが安全な場合は`Ok(())`、危険な場合は`Err`
fn validate_ip_address(ip: &IpAddr) -> Result<()> {
    match ip {
        IpAddr::V4(ipv4) => {
            // 危険なIPv4アドレス範囲を定義
            let dangerous_networks = vec![
                // localhost（127.0.0.0/8）
                "127.0.0.0/8".parse::<Ipv4Network>().unwrap(),
                // プライベートIP（10.0.0.0/8）
                "10.0.0.0/8".parse::<Ipv4Network>().unwrap(),
                // プライベートIP（172.16.0.0/12）
                "172.16.0.0/12".parse::<Ipv4Network>().unwrap(),
                // プライベートIP（192.168.0.0/16）
                "192.168.0.0/16".parse::<Ipv4Network>().unwrap(),
                // Link-local（169.254.0.0/16）
                "169.254.0.0/16".parse::<Ipv4Network>().unwrap(),
                // 0.0.0.0/8（特殊用途）
                "0.0.0.0/8".parse::<Ipv4Network>().unwrap(),
                // ブロードキャスト（255.255.255.255/32）
                "255.255.255.255/32".parse::<Ipv4Network>().unwrap(),
            ];

            // いずれかの危険な範囲に含まれるかチェック
            for network in dangerous_networks {
                if network.contains(*ipv4) {
                    return Err(anyhow!(
                        "IP address {} is not allowed (falls within blocked range {})",
                        ipv4,
                        network
                    ));
                }
            }
        }
        IpAddr::V6(ipv6) => {
            // 危険なIPv6アドレス範囲を定義
            let dangerous_networks = vec![
                // localhost（::1/128）
                "::1/128".parse::<Ipv6Network>().unwrap(),
                // Link-local（fe80::/10）
                "fe80::/10".parse::<Ipv6Network>().unwrap(),
                // Unique local（fc00::/7）
                "fc00::/7".parse::<Ipv6Network>().unwrap(),
                // 未指定アドレス（::/128）
                "::/128".parse::<Ipv6Network>().unwrap(),
            ];

            // いずれかの危険な範囲に含まれるかチェック
            for network in dangerous_networks {
                if network.contains(*ipv6) {
                    return Err(anyhow!(
                        "IP address {} is not allowed (falls within blocked range {})",
                        ipv6,
                        network
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // 安全なURLのテスト
    // ========================================

    #[test]
    fn test_validate_url_allows_public_https() {
        let url = Url::parse("https://example.com").unwrap();
        assert!(validate_url(&url).is_ok());
    }

    #[test]
    fn test_validate_url_allows_public_http() {
        let url = Url::parse("http://example.com").unwrap();
        assert!(validate_url(&url).is_ok());
    }

    #[test]
    fn test_validate_url_allows_subdomain() {
        let url = Url::parse("https://api.example.com").unwrap();
        assert!(validate_url(&url).is_ok());
    }

    // ========================================
    // スキームのテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_ftp_scheme() {
        let url = Url::parse("ftp://example.com").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_file_scheme() {
        let url = Url::parse("file:///etc/passwd").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // localhostのテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_localhost_hostname() {
        let url = Url::parse("http://localhost").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_localhost_subdomain() {
        let url = Url::parse("http://api.localhost").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_127_0_0_1() {
        let url = Url::parse("http://127.0.0.1").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_127_0_0_2() {
        // 127.0.0.0/8の範囲をテスト
        let url = Url::parse("http://127.0.0.2").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_127_1_2_3() {
        // 127.0.0.0/8の範囲をテスト
        let url = Url::parse("http://127.1.2.3").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_ipv6_localhost() {
        let url = Url::parse("http://[::1]").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_0_0_0_0() {
        let url = Url::parse("http://0.0.0.0").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // プライベートIPv4のテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_10_0_0_1() {
        let url = Url::parse("http://10.0.0.1").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_10_255_255_255() {
        let url = Url::parse("http://10.255.255.255").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_172_16_0_1() {
        let url = Url::parse("http://172.16.0.1").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_172_31_255_255() {
        let url = Url::parse("http://172.31.255.255").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_192_168_1_1() {
        let url = Url::parse("http://192.168.1.1").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // Link-localのテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_link_local_169_254_1_1() {
        let url = Url::parse("http://169.254.1.1").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // ブロードキャストのテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_broadcast_255_255_255_255() {
        let url = Url::parse("http://255.255.255.255").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // プライベートIPv6のテスト
    // ========================================

    #[test]
    fn test_validate_url_blocks_ipv6_link_local() {
        let url = Url::parse("http://[fe80::1]").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_ipv6_unique_local() {
        let url = Url::parse("http://[fc00::1]").unwrap();
        assert!(validate_url(&url).is_err());
    }

    #[test]
    fn test_validate_url_blocks_ipv6_unspecified() {
        let url = Url::parse("http://[::]").unwrap();
        assert!(validate_url(&url).is_err());
    }

    // ========================================
    // エッジケースのテスト
    // ========================================

    #[test]
    fn test_validate_url_allows_172_15_0_1() {
        // 172.16.0.0/12の範囲外（172.15は安全）
        let url = Url::parse("http://172.15.0.1").unwrap();
        assert!(validate_url(&url).is_ok());
    }

    #[test]
    fn test_validate_url_allows_172_32_0_1() {
        // 172.16.0.0/12の範囲外（172.32は安全）
        let url = Url::parse("http://172.32.0.1").unwrap();
        assert!(validate_url(&url).is_ok());
    }
}
