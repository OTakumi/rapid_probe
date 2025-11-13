use super::validate_status_code;
use crate::api_client::HttpClient;
use crate::test_case::TestCase;
use crate::test_result::TestResult;

pub async fn handle_get_request<T: HttpClient>(
    client: &T,
    test_case: &TestCase,
    mut result: TestResult,
) -> TestResult {
    // ヘッダーの準備
    let headers = test_case.request.headers.clone().unwrap_or_default();

    // リクエスト実行
    match client
        .get_with_headers(&test_case.request.url, headers)
        .await
    {
        Ok((status_code, _body)) => {
            result = result.with_status_code(status_code);
            validate_status_code(&mut result, status_code, test_case.expectations.status_code);
        }
        Err(e) => {
            result = result.with_error(format!("Request failed: {}", e));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::MockHttpClient;
    use crate::test_case::{Expectations, Request};
    use crate::test_result::TestResult;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_handle_get_request_success() {
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

        let result = TestResult::new(test_case.name.clone());

        // Act
        let result = handle_get_request(&mock_client, &test_case, result).await;

        // Assert
        assert_eq!(result.status_code, Some(200));
        assert_eq!(result.assertions.len(), 1);
        assert!(result.assertions[0].passed);
    }

    #[tokio::test]
    async fn test_handle_get_request_with_headers() {
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

        let result = TestResult::new(test_case.name.clone());

        // Act
        let result = handle_get_request(&mock_client, &test_case, result).await;

        // Assert
        assert_eq!(result.status_code, Some(401));
        assert_eq!(result.assertions.len(), 1);
        assert!(result.assertions[0].passed);
    }

    #[tokio::test]
    async fn test_handle_get_request_status_mismatch() {
        // Arrange
        let test_case = TestCase {
            name: "Status Mismatch Test".to_string(),
            description: None,
            request: Request {
                method: "GET".to_string(),
                url: "/users/999".to_string(),
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
            .withf(|url, _| url == "/users/999")
            .times(1)
            .returning(|_, _| Ok((404, r#"{"error": "Not found"}"#.to_string())));

        let result = TestResult::new(test_case.name.clone());

        // Act
        let result = handle_get_request(&mock_client, &test_case, result).await;

        // Assert
        assert_eq!(result.status_code, Some(404));
        assert_eq!(result.assertions.len(), 1);
        assert!(!result.assertions[0].passed);
        assert_eq!(result.assertions[0].expected, "200");
        assert_eq!(result.assertions[0].actual, "404");
    }

    #[tokio::test]
    async fn test_handle_get_request_failure() {
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
            .returning(|_, _| Err("Connection timeout".into()));

        let result = TestResult::new(test_case.name.clone());

        // Act
        let result = handle_get_request(&mock_client, &test_case, result).await;

        // Assert
        assert!(result.error_message.is_some());
        assert!(result.error_message.unwrap().contains("Request failed"));
    }
}
