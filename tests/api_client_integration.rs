use rapid_probe::{ApiClient, HttpClient};
use std::collections::HashMap;

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_api_client_get_with_headers_success() {
    // モックサーバーを起動
    let server = MockServer::start().await;

    // モックサーバーの期待値を設定
    Mock::given(method("GET"))
        .and(path("/api/v1/users/1"))
        .and(header("X-API-Key", "my_secret_token"))
        .and(header("Accept", "application/json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(r#"{"id": 1, "name": "Test User"}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    // テスト対象をインスタンス化
    let client = ApiClient::new(&server.uri()).expect("Failed to create client");

    // テスト用のヘッダーを準備
    let mut headers = HashMap::new();
    headers.insert("X-API-Key".to_string(), "my_secret_token".to_string());
    headers.insert("Accept".to_string(), "application/json".to_string());

    // get_with_headers メソッドを実行
    let result = client.get_with_headers("/api/v1/users/1", headers).await;

    // 結果を検証
    assert!(result.is_ok());
    let (status, body) = result.unwrap();

    // `ApiClient` がレスポンスを正しく解析できたか
    assert_eq!(status, 200);
    assert_eq!(body, r#"{"id": 1, "name": "Test User"}"#);
}

#[tokio::test]
async fn test_api_client_handles_404_not_found() {
    // サーバー起動
    let server = MockServer::start().await;

    // 404を返すルールを定義
    Mock::given(path("/api/not-found"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Resource was not found"))
        .mount(&server)
        .await;

    // クライアント作成
    let client = ApiClient::new(&server.uri()).unwrap();

    // 実行
    let result = client
        .get_with_headers("/api/not-found", HashMap::new())
        .await;

    // 検証
    // 404でも通信自体は 'Ok' で、タプルが返る
    assert!(result.is_ok());
    let (status, body) = result.unwrap();
    assert_eq!(status, 404);
    assert_eq!(body, "Resource was not found");
}
