//! HTTPクライアント統合テスト（実API使用）
//!
//! このモジュールは、ApiClientの実際のHTTP通信機能を検証します。
//! テスト対象API: https://jsonplaceholder.typicode.com (公開テストAPI)
//!
//! # テスト方針
//!
//! - 実際の外部APIを使用してネットワーク通信を検証
//! - Rapid Probeが不特定のAPIをテストできることを確認
//! - GETおよびPOSTメソッドの動作を検証

mod helpers;

use helpers::{create_headers, create_test_client};
use rapid_probe::HttpClient;
use std::collections::HashMap;

const BASE_URL: &str = "https://jsonplaceholder.typicode.com";

/// GETリクエストで単一の記事を取得できることをテスト
///
/// # テスト内容
///
/// - JSONPlaceholder APIに対してGETリクエストを送信
/// - 記事ID=1のデータを取得
///
/// # 検証項目
///
/// - ステータスコード200が返されること
/// - レスポンスボディがJSON形式であること
/// - レスポンスに"userId"フィールドが含まれること
#[tokio::test]
async fn test_get_single_post() {
    // Arrange: テスト用クライアントを作成
    let client = create_test_client(BASE_URL);

    // Act: 記事ID=1を取得
    let result = client.get_with_headers("/posts/1", HashMap::new()).await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "Request failed: {:?}", result.err());
    let (status, body) = result.unwrap();
    assert_eq!(status, 200, "Expected status code 200");
    assert!(
        body.contains("userId"),
        "Response should contain userId field"
    );
    assert!(body.contains("\"id\""), "Response should contain id field");
}

/// GETリクエストでカスタムヘッダーを送信できることをテスト
///
/// # テスト内容
///
/// - カスタムヘッダー（Accept: application/json）を付与してGETリクエストを送信
/// - JSONPlaceholder APIは任意のヘッダーを受け入れる
///
/// # 検証項目
///
/// - ステータスコード200が返されること
/// - ヘッダーを含むリクエストが正常に処理されること
#[tokio::test]
async fn test_get_with_custom_headers() {
    // Arrange: テスト用クライアントとヘッダーを準備
    let client = create_test_client(BASE_URL);
    let headers = create_headers(&[("Accept", "application/json")]);

    // Act: カスタムヘッダー付きでGETリクエストを実行
    let result = client.get_with_headers("/posts/1", headers).await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "Request with headers failed");
    let (status, _body) = result.unwrap();
    assert_eq!(status, 200, "Expected status code 200");
}

/// GETリクエストで存在しないリソースに対する404エラーを正しく処理できることをテスト
///
/// # テスト内容
///
/// - 存在しない記事ID（999）に対してGETリクエストを送信
/// - 404レスポンスを正しくハンドリング
///
/// # 検証項目
///
/// - ステータスコード404が返されること
/// - レスポンスボディが空のJSONオブジェクト"{}"であること
/// - 通信エラーではなく正常なレスポンスとして処理されること
#[tokio::test]
async fn test_get_not_found() {
    // Arrange: テスト用クライアントを作成
    let client = create_test_client(BASE_URL);

    // Act: 存在しない記事IDを取得
    let result = client.get_with_headers("/posts/999", HashMap::new()).await;

    // Assert: 404レスポンスを検証
    assert!(result.is_ok(), "404 should be handled as Ok response");
    let (status, body) = result.unwrap();
    assert_eq!(status, 404, "Expected status code 404");
    assert_eq!(body, "{}", "JSONPlaceholder returns empty object for 404");
}

/// GETリクエストで複数の記事を取得できることをテスト
///
/// # テスト内容
///
/// - /posts エンドポイントから記事一覧を取得
/// - 複数のリソースを含むレスポンスを検証
///
/// # 検証項目
///
/// - ステータスコード200が返されること
/// - レスポンスがJSON配列であること（"["で始まる）
/// - 複数の記事が含まれること
#[tokio::test]
async fn test_get_multiple_posts() {
    // Arrange: テスト用クライアントを作成
    let client = create_test_client(BASE_URL);

    // Act: 記事一覧を取得
    let result = client.get_with_headers("/posts", HashMap::new()).await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "Request failed");
    let (status, body) = result.unwrap();
    assert_eq!(status, 200, "Expected status code 200");
    assert!(body.starts_with('['), "Response should be a JSON array");
    assert!(body.len() > 100, "Response should contain multiple posts");
}

/// POSTリクエストで新規記事を作成できることをテスト
///
/// # テスト内容
///
/// - JSONPlaceholder APIに対してPOSTリクエストを送信
/// - 新規記事データをJSON形式で送信
///
/// # 検証項目
///
/// - ステータスコード201 (Created) が返されること
/// - レスポンスボディにidフィールドが含まれること
/// - 送信したデータ（title, body, userId）がレスポンスに反映されること
#[tokio::test]
async fn test_post_create_new_post() {
    // Arrange: テスト用クライアントと送信データを準備
    let client = create_test_client(BASE_URL);
    let headers = create_headers(&[("Content-Type", "application/json")]);
    let body = r#"{
        "title": "Test Post Title",
        "body": "This is a test post content",
        "userId": 1
    }"#;

    // Act: POSTリクエストで新規記事を作成
    let result = client
        .post_with_body("/posts", headers, body.to_string())
        .await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "POST request failed: {:?}", result.err());
    let (status, response_body) = result.unwrap();
    assert_eq!(status, 201, "Expected status code 201 Created");
    assert!(
        response_body.contains("\"id\""),
        "Response should contain id field"
    );
    assert!(
        response_body.contains("Test Post Title"),
        "Response should contain the posted title"
    );
}

/// POSTリクエストでカスタムヘッダーを送信できることをテスト
///
/// # テスト内容
///
/// - 複数のカスタムヘッダーを含むPOSTリクエストを送信
/// - Content-TypeとAuthorizationヘッダーの両方を設定
///
/// # 検証項目
///
/// - ステータスコード201が返されること
/// - ヘッダーを含むPOSTリクエストが正常に処理されること
#[tokio::test]
async fn test_post_with_custom_headers() {
    // Arrange: 複数のカスタムヘッダーを準備
    let client = create_test_client(BASE_URL);
    let headers = create_headers(&[
        ("Content-Type", "application/json"),
        ("Authorization", "Bearer test-token"),
    ]);
    let body = r#"{"title": "Header Test", "body": "Testing headers", "userId": 1}"#;

    // Act: カスタムヘッダー付きでPOSTリクエストを実行
    let result = client
        .post_with_body("/posts", headers, body.to_string())
        .await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "POST with headers failed");
    let (status, _response_body) = result.unwrap();
    assert_eq!(status, 201, "Expected status code 201");
}

/// POSTリクエストで空のボディを送信できることをテスト
///
/// # テスト内容
///
/// - 空の文字列をボディとして送信
/// - APIが空のボディを受け入れることを確認
///
/// # 検証項目
///
/// - リクエストが正常に完了すること
/// - ステータスコード201が返されること
#[tokio::test]
async fn test_post_with_empty_body() {
    // Arrange: 空のボディを準備
    let client = create_test_client(BASE_URL);
    let headers = create_headers(&[("Content-Type", "application/json")]);
    let body = String::new();

    // Act: 空のボディでPOSTリクエストを実行
    let result = client.post_with_body("/posts", headers, body).await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "POST with empty body failed");
    let (status, _response_body) = result.unwrap();
    assert_eq!(status, 201, "Expected status code 201");
}

/// POSTリクエストで複雑なJSONデータを送信できることをテスト
///
/// # テスト内容
///
/// - ネストされたJSONオブジェクトを含むデータを送信
/// - 複数のフィールドを持つ複雑なペイロードを検証
///
/// # 検証項目
///
/// - ステータスコード201が返されること
/// - レスポンスボディに送信したデータが含まれること
#[tokio::test]
async fn test_post_with_complex_json() {
    // Arrange: 複雑なJSONデータを準備
    let client = create_test_client(BASE_URL);
    let headers = create_headers(&[("Content-Type", "application/json")]);
    let body = r#"{
        "title": "Complex Post",
        "body": "This post has special characters: 日本語, émojis 🚀",
        "userId": 42,
        "tags": ["test", "integration"],
        "metadata": {
            "version": "1.0",
            "author": "Test Suite"
        }
    }"#;

    // Act: 複雑なJSONでPOSTリクエストを実行
    let result = client
        .post_with_body("/posts", headers, body.to_string())
        .await;

    // Assert: レスポンスを検証
    assert!(result.is_ok(), "POST with complex JSON failed");
    let (status, response_body) = result.unwrap();
    assert_eq!(status, 201, "Expected status code 201");
    assert!(
        response_body.contains("Complex Post"),
        "Response should contain the posted title"
    );
}
