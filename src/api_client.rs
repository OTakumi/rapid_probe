use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, instrument};
use url::Url;

use crate::ssrf_protection;

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

    async fn post_with_body(
        &self,
        url: &str,
        headers: HashMap<String, String>,
        body: String,
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
            ssrf_protection::validate_url(&base_url)?;
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

    #[instrument(skip(self, headers, body), fields(url = %url, header_count = headers.len(), body_len = body.len()))]
    async fn post_with_body(
        &self,
        url: &str,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<(u16, String)> {
        debug!("Starting POST request to: {}", url);

        // urlを結合する
        let full_url = self.base_url.join(url).with_context(|| {
            format!(
                "failed to build URL from base {} and path {}",
                self.base_url, url
            )
        })?;

        debug!("Full URL: {}", full_url);
        debug!("Request headers: {:?}", headers);
        debug!("Request body length: {} bytes", body.len());

        // headerをリクエストに適用する
        let mut request_builder = self.client.post(full_url);

        for (k, v) in headers {
            request_builder = request_builder.header(k, v);
        }

        // bodyを設定
        request_builder = request_builder.body(body);

        // リクエストし、レスポンスを受け取る
        let response = request_builder
            .send()
            .await
            .context("HTTP request failed")?;

        // レスポンスの内容を分解する
        let status = response.status().as_u16();
        info!("Received response with status: {}", status);

        let response_body = response
            .text()
            .await
            .context("failed to read response body")?;

        debug!("Response body length: {} bytes", response_body.len());

        Ok((status, response_body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_client_post_with_body_basic() {
        // Arrange
        let client = ApiClient::new("https://jsonplaceholder.typicode.com").unwrap();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = r#"{"title": "test", "body": "content", "userId": 1}"#.to_string();

        // Act
        let result = client.post_with_body("/posts", headers, body).await;

        // Assert
        assert!(result.is_ok(), "POSTリクエストが成功すべき");
        let (status, response_body) = result.unwrap();
        assert!(
            status >= 200 && status < 300,
            "ステータスコードが2xxであるべき"
        );
        assert!(!response_body.is_empty(), "レスポンスボディが空でないべき");
    }
}
