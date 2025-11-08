use rapid_probe::{ApiClient, HttpClient};
use std::collections::HashMap;

#[tokio::test]
async fn test_simple_get_request() {
    // Arrange
    let client = ApiClient::new("https://httpbin.org").unwrap();

    // Act
    let (status, body) = client
        .get_with_headers("/get", HashMap::new())
        .await
        .unwrap();

    // Assert
    assert_eq!(status, 200);
    assert!(body.contains("httpbin.org"));
}

#[tokio::test]
async fn test_get_request_with_headers() {
    // Arrange
    let client = ApiClient::new("https://httpbin.org").unwrap();
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "test-value".to_string());

    // Act
    let (status, body) = client.get_with_headers("/headers", headers).await.unwrap();

    // Assert
    assert_eq!(status, 200);
    assert!(body.contains("X-Custom-Header"));
    assert!(body.contains("test-value"));
}

#[tokio::test]
async fn test_post_request_with_data() {
    // TODO: POSTメソッドのサポートを追加後に実装
    // 現在のApiClientはGETのみサポート
}
