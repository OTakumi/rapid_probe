use rapid_probe::{ApiClient, HttpClient};
use std::collections::HashMap;

#[tokio::test]
async fn test_simple_get_request() {
    // Arrange
    let client = ApiClient::new("https://jsonplaceholder.typicode.com").unwrap();

    // Act
    let (status, body) = client
        .get_with_headers("/posts/1", HashMap::new())
        .await
        .unwrap();

    // Assert
    assert_eq!(status, 200);
    assert!(body.contains("userId"));
    assert!(body.contains("title"));
}

#[tokio::test]
async fn test_get_request_with_headers() {
    // Arrange
    let client = ApiClient::new("https://jsonplaceholder.typicode.com").unwrap();
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("User-Agent".to_string(), "RapidProbe/1.0".to_string());

    // Act
    let (status, body) = client.get_with_headers("/posts", headers).await.unwrap();

    // Assert
    assert_eq!(status, 200);
    // JSONPlaceholder returns an array of posts
    assert!(body.starts_with('['));
    assert!(body.contains("userId"));
}

#[tokio::test]
async fn test_get_request_not_found() {
    // Arrange
    let client = ApiClient::new("https://jsonplaceholder.typicode.com").unwrap();

    // Act
    let (status, body) = client
        .get_with_headers("/posts/999", HashMap::new())
        .await
        .unwrap();

    // Assert
    // JSONPlaceholder returns 404 for non-existent posts
    assert_eq!(status, 404);
    assert_eq!(body.trim(), "{}");
}

#[tokio::test]
async fn test_post_request_with_data() {
    // TODO: POSTメソッドのサポートを追加後に実装
    // 現在のApiClientはGETのみサポート
}
