use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::api_client::HttpClient;
use crate::test_case::TestCase;

/// HTTPレスポンス構造体
///
/// Strategyから返されるレスポンスを表現
#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// HTTPメソッドハンドラーのStrategy トレイト
///
/// 各HTTPメソッド（GET, POST, PUT等）の実装はこのトレイトを実装する
#[async_trait]
pub trait HttpMethodStrategy: Send + Sync + std::fmt::Debug {
    /// 指定されたテストケースに対してHTTPリクエストを実行
    async fn execute(&self, client: &dyn HttpClient, test_case: &TestCase) -> Result<HttpResponse>;

    /// このStrategyが処理するHTTPメソッド名を返す
    fn method_name(&self) -> &'static str;
}

/// HTTPメソッドStrategyを生成するファクトリ
pub struct StrategyFactory;

impl StrategyFactory {
    /// 指定されたHTTPメソッドに対応するStrategyを取得
    pub fn get_strategy(method: &str) -> Result<Box<dyn HttpMethodStrategy>> {
        use crate::http::get_strategy::GetStrategy;

        match method.to_uppercase().as_str() {
            "GET" => Ok(Box::new(GetStrategy)),
            _ => anyhow::bail!("HTTP method '{}' is not supported yet", method),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_factory_returns_get_strategy_uppercase() {
        // Arrange & Act
        let result = StrategyFactory::get_strategy("GET");

        // Assert
        assert!(result.is_ok(), "GETメソッドに対してOkを返すべき");
        let strategy = result.unwrap();
        assert_eq!(strategy.method_name(), "GET");
    }

    #[test]
    fn test_strategy_factory_returns_get_strategy_lowercase() {
        // Arrange & Act
        let result = StrategyFactory::get_strategy("get");

        // Assert
        assert!(result.is_ok(), "小文字のgetメソッドに対してOkを返すべき");
        let strategy = result.unwrap();
        assert_eq!(strategy.method_name(), "GET");
    }

    #[test]
    fn test_strategy_factory_returns_error_for_unsupported_method() {
        // Arrange & Act
        let result = StrategyFactory::get_strategy("POST");

        // Assert
        assert!(result.is_err(), "未サポートのメソッドに対してErrを返すべき");
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("not supported") || err_msg.contains("not implemented"),
            "エラーメッセージに未サポートの旨を含むべき: {}",
            err_msg
        );
    }

    #[test]
    fn test_strategy_factory_returns_error_for_delete() {
        // Arrange & Act
        let result = StrategyFactory::get_strategy("DELETE");

        // Assert
        assert!(result.is_err(), "DELETEメソッドに対してErrを返すべき");
    }

    #[test]
    fn test_http_response_equality() {
        // Arrange
        let mut headers1 = HashMap::new();
        headers1.insert("Content-Type".to_string(), "application/json".to_string());

        let mut headers2 = HashMap::new();
        headers2.insert("Content-Type".to_string(), "application/json".to_string());

        let response1 = HttpResponse {
            status_code: 200,
            headers: headers1,
            body: "test".to_string(),
        };

        let response2 = HttpResponse {
            status_code: 200,
            headers: headers2,
            body: "test".to_string(),
        };

        // Assert
        assert_eq!(response1, response2);
    }

    #[test]
    fn test_http_response_clone() {
        // Arrange
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let response = HttpResponse {
            status_code: 200,
            headers,
            body: "test".to_string(),
        };

        // Act
        let cloned = response.clone();

        // Assert
        assert_eq!(response, cloned);
    }
}
