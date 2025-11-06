use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use url::{ParseError, Url};

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
    ) -> Result<(u16, String), Box<dyn std::error::Error>>;
}

pub struct ApiClient {
    base_url: Url,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url_str: &str) -> Result<Self, ParseError> {
        // 文字列をUrlオブジェクトにパース
        let base_url = Url::parse(base_url_str)?;

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
    ) -> Result<(u16, String), Box<dyn std::error::Error>> {
        // urlを結合する
        let full_url = self.base_url.join(url)?;

        // headerをリクエストに適用する
        let mut request_builder = self.client.get(full_url);

        for (k, v) in headers {
            request_builder = request_builder.header(k, v);
        }

        // リクエストし、レスポンスを受け取る
        let response = request_builder.send().await?;

        // レスポンスの内容を分解する
        let status = response.status().as_u16();
        let body = response.text().await?;

        Ok((status, body))
    }
}
