//! 統合テスト用の共通ヘルパー関数
//!
//! このモジュールは、統合テスト全体で使用される共通のユーティリティ関数を提供します。

use rapid_probe::ApiClient;
use std::collections::HashMap;

/// テスト用のApiClientを作成する
///
/// # Arguments
///
/// * `base_url` - ベースURL（通常はWireMockサーバーのURI）
///
/// # Returns
///
/// テスト用のApiClientインスタンス（URL検証をスキップ）
///
/// # Panics
///
/// ApiClientの作成に失敗した場合にパニックします
pub fn create_test_client(base_url: &str) -> ApiClient {
    ApiClient::new_for_testing(base_url).expect("Failed to create test ApiClient")
}

/// HTTPヘッダーのHashMapを簡単に作成するヘルパー関数
///
/// # Arguments
///
/// * `headers` - (キー, 値)のタプルのスライス
///
/// # Returns
///
/// HashMap<String, String>形式のヘッダー
///
/// # Examples
///
/// ```ignore
/// let headers = create_headers(&[
///     ("Content-Type", "application/json"),
///     ("Authorization", "Bearer token"),
/// ]);
/// ```
pub fn create_headers(headers: &[(&str, &str)]) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// HTTPレスポンスのステータスコードとボディを検証する
///
/// # Arguments
///
/// * `result` - HttpClientのメソッドから返されたResult<(u16, String)>
/// * `expected_status` - 期待されるステータスコード
/// * `expected_body` - 期待されるレスポンスボディ
///
/// # Panics
///
/// - resultがErrの場合
/// - ステータスコードが一致しない場合
/// - ボディが一致しない場合
pub fn assert_response(
    result: anyhow::Result<(u16, String)>,
    expected_status: u16,
    expected_body: &str,
) {
    assert!(result.is_ok(), "Request failed: {:?}", result.err());
    let (status, body) = result.unwrap();
    assert_eq!(status, expected_status, "Status code mismatch");
    assert_eq!(body, expected_body, "Response body mismatch");
}
