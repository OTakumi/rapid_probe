use anyhow::Result;
use async_trait::async_trait;

use crate::api_client::HttpClient;
use crate::http::strategy::{HttpMethodStrategy, HttpResponse};
use crate::test_case::TestCase;

/// GETリクエストを処理するStrategy
#[derive(Debug)]
pub struct GetStrategy;

#[async_trait]
impl HttpMethodStrategy for GetStrategy {
    async fn execute(&self, client: &dyn HttpClient, test_case: &TestCase) -> Result<HttpResponse> {
        // ヘッダーの準備（test_caseにあればclone、なければ空）
        let headers = test_case.request.headers.clone().unwrap_or_default();

        // リクエスト実行
        let (status_code, body) = client
            .get_with_headers(&test_case.request.url, headers)
            .await
            .map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

        // レスポンスを返す
        Ok(HttpResponse {
            status_code,
            headers: std::collections::HashMap::new(), // TODO: 将来的にレスポンスヘッダーを抽出
            body,
        })
    }

    fn method_name(&self) -> &'static str {
        "GET"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::MockHttpClient;
    use crate::test_case::{Expectations, Request};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_strategy_execute_success() {
        // Arrange
        let test_case = TestCase {
            name: "Get User Test".to_string(),
            description: Some("Test getting a user".to_string()),
            request: Request {
                method: "GET".to_string(),
                url: "/users/1".to_string(),
                headers: None,
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 200,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(|url, _headers| url == "/users/1")
            .times(1)
            .returning(|_, _| Ok((200, r#"{"id": 1, "name": "John Doe"}"#.to_string())));

        let strategy = GetStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok for successful request");
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, r#"{"id": 1, "name": "John Doe"}"#);
    }

    #[tokio::test]
    async fn test_get_strategy_execute_with_headers() {
        // Arrange
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let test_case = TestCase {
            name: "Auth Test".to_string(),
            description: None,
            request: Request {
                method: "GET".to_string(),
                url: "/protected".to_string(),
                headers: Some(headers.clone()),
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 401,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(move |url, req_headers| {
                url == "/protected"
                    && req_headers.get("Authorization") == Some(&"Bearer token123".to_string())
                    && req_headers.get("Content-Type") == Some(&"application/json".to_string())
            })
            .times(1)
            .returning(|_, _| Ok((401, r#"{"error": "Unauthorized"}"#.to_string())));

        let strategy = GetStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok for request with headers");
        let response = result.unwrap();
        assert_eq!(response.status_code, 401);
        assert_eq!(response.body, r#"{"error": "Unauthorized"}"#);
    }

    #[tokio::test]
    async fn test_get_strategy_execute_not_found() {
        // Arrange
        let test_case = TestCase {
            name: "Not Found Test".to_string(),
            description: None,
            request: Request {
                method: "GET".to_string(),
                url: "/users/999".to_string(),
                headers: None,
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 404,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(|url, _| url == "/users/999")
            .times(1)
            .returning(|_, _| Ok((404, r#"{"error": "Not found"}"#.to_string())));

        let strategy = GetStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok even for 404");
        let response = result.unwrap();
        assert_eq!(response.status_code, 404);
        assert_eq!(response.body, r#"{"error": "Not found"}"#);
    }

    #[tokio::test]
    async fn test_get_strategy_execute_request_failure() {
        // Arrange
        let test_case = TestCase {
            name: "Request Failure Test".to_string(),
            description: None,
            request: Request {
                method: "GET".to_string(),
                url: "/error".to_string(),
                headers: None,
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 200,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(|url, _| url == "/error")
            .times(1)
            .returning(|_, _| Err(anyhow::anyhow!("Connection timeout")));

        let strategy = GetStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_err(), "Should return Err for failed request");
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Connection timeout") || err_msg.contains("Request failed"));
    }

    #[test]
    fn test_get_strategy_method_name() {
        // Arrange
        let strategy = GetStrategy;

        // Act
        let method = strategy.method_name();

        // Assert
        assert_eq!(method, "GET");
    }

    #[tokio::test]
    async fn test_get_strategy_execute_empty_headers() {
        // Arrange
        let test_case = TestCase {
            name: "Empty Headers Test".to_string(),
            description: None,
            request: Request {
                method: "GET".to_string(),
                url: "/test".to_string(),
                headers: None,
                body_file: None,
                body: None,
            },
            expectations: Expectations {
                status_code: 200,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_get_with_headers()
            .withf(|url, headers| url == "/test" && headers.is_empty())
            .times(1)
            .returning(|_, _| Ok((200, "OK".to_string())));

        let strategy = GetStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(
            result.is_ok(),
            "Should return Ok for request without headers"
        );
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, "OK");
    }
}
