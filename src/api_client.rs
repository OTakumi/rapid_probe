use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, instrument};
use url::Url;

#[derive(Debug)]
pub struct ApiResponse {
    pub status: u16,
    pub body: Value,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get_with_headers(
        &self,
        url: &str,
        headers: HashMap<String, String>,
    ) -> Result<(u16, String)>;
}

pub struct ApiClient {
    base_url: Url,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url_str: &str) -> Result<Self> {
        Self::new_internal(base_url_str, true)
    }

    /// テスト用のコンストラクタ（URL検証をスキップ）
    ///
    /// 統合テストでlocalhostを使用する場合に使用します。
    /// 本番環境では使用しないでください。
    #[doc(hidden)]
    pub fn new_for_testing(base_url_str: &str) -> Result<Self> {
        Self::new_internal(base_url_str, false)
    }

    fn new_internal(base_url_str: &str, validate: bool) -> Result<Self> {
        debug!("Creating API client with base URL: {}", base_url_str);

        // 文字列をUrlオブジェクトにパース
        let base_url = Url::parse(base_url_str)
            .with_context(|| format!("invalid base URL: {}", base_url_str))?;

        // URL検証: SSRF攻撃を防ぐ
        if validate {
            Self::validate_url(&base_url)?;
        } else {
            debug!("URL validation skipped (testing mode)");
        }

        // タイムアウト設定でHTTPクライアントを構築
        let client = Client::builder()
            .timeout(Duration::from_secs(30)) // 全体のタイムアウト: 30秒
            .connect_timeout(Duration::from_secs(10)) // 接続タイムアウト: 10秒
            .build()
            .context("failed to build HTTP client")?;

        info!("API client created successfully with timeouts configured");
        Ok(Self { base_url, client })
    }

    /// URLのバリデーション
    ///
    /// SSRF（Server-Side Request Forgery）攻撃を防ぐため、
    /// プライベートIPアドレスやlocalhostへのアクセスを制限
    fn validate_url(url: &Url) -> Result<()> {
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

        // localhostの検出
        if host == "localhost"
            || host == "127.0.0.1"
            || host == "::1"
            || host.ends_with(".localhost")
        {
            return Err(anyhow!(
                "localhost URLs are not allowed for security reasons: {}",
                host
            ));
        }

        // プライベートIPアドレスの検出（簡易版）
        // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        if host.starts_with("10.")
            || host.starts_with("192.168.")
            || (host.starts_with("172.")
                && host
                    .split('.')
                    .nth(1)
                    .and_then(|s| s.parse::<u8>().ok())
                    .map(|n| (16..=31).contains(&n))
                    .unwrap_or(false))
        {
            return Err(anyhow!(
                "private IP addresses are not allowed for security reasons: {}",
                host
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl HttpClient for ApiClient {
    #[instrument(skip(self, headers), fields(url = %url, header_count = headers.len()))]
    async fn get_with_headers(
        &self,
        url: &str,
        headers: HashMap<String, String>,
    ) -> Result<(u16, String)> {
        debug!("Starting GET request to: {}", url);

        // urlを結合する
        let full_url = self.base_url.join(url).with_context(|| {
            format!(
                "failed to build URL from base {} and path {}",
                self.base_url, url
            )
        })?;

        debug!("Full URL: {}", full_url);
        debug!("Request headers: {:?}", headers);

        // headerをリクエストに適用する
        let mut request_builder = self.client.get(full_url);

        for (k, v) in headers {
            request_builder = request_builder.header(k, v);
        }

        // リクエストし、レスポンスを受け取る
        let response = request_builder
            .send()
            .await
            .context("HTTP request failed")?;

        // レスポンスの内容を分解する
        let status = response.status().as_u16();
        info!("Received response with status: {}", status);

        let body = response
            .text()
            .await
            .context("failed to read response body")?;

        debug!("Response body length: {} bytes", body.len());

        Ok((status, body))
    }
}
