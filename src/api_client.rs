use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
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
        // 文字列をUrlオブジェクトにパース
        let base_url = Url::parse(base_url_str)
            .with_context(|| format!("invalid base URL: {}", base_url_str))?;

        Ok(Self {
            base_url,
            client: Client::new(),
        })
    }
}

#[async_trait]
impl HttpClient for ApiClient {
    async fn get_with_headers(
        &self,
        url: &str,
        headers: HashMap<String, String>,
    ) -> Result<(u16, String)> {
        // urlを結合する
        let full_url = self.base_url.join(url).with_context(|| {
            format!(
                "failed to build URL from base {} and path {}",
                self.base_url, url
            )
        })?;

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
        let body = response
            .text()
            .await
            .context("failed to read response body")?;

        Ok((status, body))
    }
}
