use anyhow::Result;
use async_trait::async_trait;

use crate::api_client::HttpClient;
use crate::http::strategy::{HttpMethodStrategy, HttpResponse};
use crate::test_case::TestCase;

/// POSTリクエストを処理するStrategy
#[derive(Debug)]
pub struct PostStrategy;

#[async_trait]
impl HttpMethodStrategy for PostStrategy {
    async fn execute(&self, client: &dyn HttpClient, test_case: &TestCase) -> Result<HttpResponse> {
        // ヘッダーの準備（test_caseにあればclone、なければ空）
        let headers = test_case.request.headers.clone().unwrap_or_default();

        // ボディの準備
        let body = if let Some(body_value) = &test_case.request.body {
            // JSON Valueを文字列に変換
            body_value.to_string()
        } else if let Some(body_file) = &test_case.request.body_file {
            // ファイルからボディを読み込む
            std::fs::read_to_string(body_file)
                .map_err(|e| anyhow::anyhow!("failed to read body file '{}': {}", body_file, e))?
        } else {
            // bodyもbody_fileもない場合は空文字列
            String::new()
        };

        // POSTリクエスト実行
        let (status_code, response_body) = client
            .post_with_body(&test_case.request.url, headers, body)
            .await
            .map_err(|e| anyhow::anyhow!("Request failed: {}", e))?;

        // レスポンスを返す
        Ok(HttpResponse {
            status_code,
            headers: std::collections::HashMap::new(), // TODO: 将来的にレスポンスヘッダーを抽出
            body: response_body,
        })
    }

    fn method_name(&self) -> &'static str {
        "POST"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::MockHttpClient;
    use crate::test_case::{Expectations, Request};
    use serde_json::json;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_post_strategy_execute_with_json_body() {
        // Arrange
        let test_case = TestCase {
            name: "Create User".to_string(),
            description: Some("Test creating a user".to_string()),
            request: Request {
                method: "POST".to_string(),
                url: "/users".to_string(),
                headers: None,
                body: Some(json!({"name": "John", "age": 30})),
                body_file: None,
            },
            expectations: Expectations {
                status_code: 201,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_post_with_body()
            .withf(|url, _headers, body| {
                url == "/users" && body.contains("John") && body.contains("30")
            })
            .times(1)
            .returning(|_, _, _| {
                Ok((201, r#"{"id": 123, "name": "John", "age": 30}"#.to_string()))
            });

        let strategy = PostStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(
            result.is_ok(),
            "Should return Ok for successful POST request"
        );
        let response = result.unwrap();
        assert_eq!(response.status_code, 201);
        assert!(response.body.contains("123"));
    }

    #[tokio::test]
    async fn test_post_strategy_execute_with_headers() {
        // Arrange
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let test_case = TestCase {
            name: "Create Post".to_string(),
            description: None,
            request: Request {
                method: "POST".to_string(),
                url: "/posts".to_string(),
                headers: Some(headers.clone()),
                body: Some(json!({"title": "Test", "content": "Test content"})),
                body_file: None,
            },
            expectations: Expectations {
                status_code: 201,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_post_with_body()
            .withf(move |url, req_headers, body| {
                url == "/posts"
                    && req_headers.get("Authorization") == Some(&"Bearer token123".to_string())
                    && req_headers.get("Content-Type") == Some(&"application/json".to_string())
                    && body.contains("Test")
            })
            .times(1)
            .returning(|_, _, _| Ok((201, r#"{"id": 456}"#.to_string())));

        let strategy = PostStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok for POST with headers");
        let response = result.unwrap();
        assert_eq!(response.status_code, 201);
    }

    #[tokio::test]
    async fn test_post_strategy_execute_with_empty_body() {
        // Arrange
        let test_case = TestCase {
            name: "Empty Body POST".to_string(),
            description: None,
            request: Request {
                method: "POST".to_string(),
                url: "/endpoint".to_string(),
                headers: None,
                body: None,
                body_file: None,
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
            .expect_post_with_body()
            .withf(|url, _, body| url == "/endpoint" && body.is_empty())
            .times(1)
            .returning(|_, _, _| Ok((200, "OK".to_string())));

        let strategy = PostStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok for POST with empty body");
        let response = result.unwrap();
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn test_post_strategy_method_name() {
        // Arrange
        let strategy = PostStrategy;

        // Act
        let method = strategy.method_name();

        // Assert
        assert_eq!(method, "POST");
    }

    #[tokio::test]
    async fn test_post_strategy_execute_with_body_file() {
        // Arrange
        use std::io::Write;
        use tempfile::NamedTempFile;

        // 一時ファイルを作成してJSONボディを書き込む
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{"username": "testuser", "email": "test@example.com"}"#;
        temp_file.write_all(json_content.as_bytes()).unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let test_case = TestCase {
            name: "Body File Test".to_string(),
            description: None,
            request: Request {
                method: "POST".to_string(),
                url: "/users".to_string(),
                headers: None,
                body: None,
                body_file: Some(file_path.clone()),
            },
            expectations: Expectations {
                status_code: 201,
                headers: None,
                response: None,
                performance: None,
                custom_assertions: None,
            },
            variables: None,
        };

        let mut mock_client = MockHttpClient::new();
        mock_client
            .expect_post_with_body()
            .withf(|url, _, body| {
                url == "/users" && body.contains("testuser") && body.contains("test@example.com")
            })
            .times(1)
            .returning(|_, _, _| Ok((201, r#"{"id": 789}"#.to_string())));

        let strategy = PostStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_ok(), "Should return Ok for POST with body_file");
        let response = result.unwrap();
        assert_eq!(response.status_code, 201);
    }

    #[tokio::test]
    async fn test_post_strategy_execute_request_failure() {
        // Arrange
        let test_case = TestCase {
            name: "Request Failure Test".to_string(),
            description: None,
            request: Request {
                method: "POST".to_string(),
                url: "/error".to_string(),
                headers: None,
                body: Some(json!({"test": "data"})),
                body_file: None,
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
            .expect_post_with_body()
            .withf(|url, _, _| url == "/error")
            .times(1)
            .returning(|_, _, _| Err(anyhow::anyhow!("Connection timeout")));

        let strategy = PostStrategy;

        // Act
        let result = strategy.execute(&mock_client, &test_case).await;

        // Assert
        assert!(result.is_err(), "Should return Err for failed request");
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Connection timeout") || err_msg.contains("Request failed"));
    }
}
