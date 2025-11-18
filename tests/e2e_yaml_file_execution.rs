//! E2Eテスト：YAMLファイル実行テスト
//!
//! このモジュールは、Rapid Probeの完全なE2Eテストを提供します。
//! cargo runサブプロセスを使用して、実際のCLI動作を検証します。
//!
//! # テスト方針
//!
//! - 実際のバイナリを実行してCLI全体の動作を検証
//! - YAMLファイルベースのテスト実行を確認
//! - 標準出力・標準エラー出力の検証
//! - 終了コードの検証

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// YAMLファイルからテストケースを実行できることをテスト
///
/// # テスト内容
///
/// - YAMLファイルに定義されたGETリクエストのテストケースを実行
/// - JSONPlaceholder APIに対して実際のリクエストを送信
/// - コマンドライン引数でYAMLファイルを指定
///
/// # 検証項目
///
/// - コマンドが正常終了すること（exit code 0）
/// - 標準出力に「テストケースファイルを読み込み中」が含まれること
/// - 標準出力に「✓ PASS」が含まれること
/// - テスト結果サマリーが正しく表示されること（合計1、成功1、失敗0）
#[test]
fn test_yaml_file_execution() {
    // Arrange: YAMLテストファイルの作成
    let yaml_content = r#"
name: "Integration Test Suite"
description: "Test for YAML execution"
base_url: "https://jsonplaceholder.typicode.com"
tests:
  - name: "Get Request Test"
    description: "Simple GET request"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Assert: 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success());
    assert!(stdout.contains("テストケースファイルを読み込み中"));
    assert!(stdout.contains("✓ PASS Get Request Test"));
    assert!(stdout.contains("合計: 1 / 成功: 1 / 失敗: 0"));
}

/// YAMLファイルで失敗するテストケースが正しく処理されることをテスト
///
/// # テスト内容
///
/// - 成功するテストと失敗するテストを含むYAMLファイルを実行
/// - 期待されるステータスコードと実際のステータスコードが異なるケースを検証
///
/// # 検証項目
///
/// - コマンドが異常終了すること（exit code 1）
/// - 成功したテストに「✓ PASS」が表示されること
/// - 失敗したテストに「✗ FAIL」が表示されること
/// - テスト結果サマリーが正しいこと（合計2、成功1、失敗1）
#[test]
fn test_yaml_file_with_failing_test() {
    // Arrange: 失敗するテストを含むYAMLファイルを作成
    let yaml_content = r#"
name: "Failing Test Suite"
base_url: "https://jsonplaceholder.typicode.com"
tests:
  - name: "Expected 200 Test"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200
  - name: "Expected 200 but get 404"
    request:
      method: "GET"
      url: "/posts/999"
    expectations:
      status_code: 200
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Assert: 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);

    // 失敗したテストがある場合、exitコードは1
    assert!(!output.status.success());
    assert!(stdout.contains("✓ PASS Expected 200 Test"));
    assert!(stdout.contains("✗ FAIL Expected 200 but get 404"));
    assert!(stdout.contains("合計: 2 / 成功: 1 / 失敗: 1"));
}

/// Verboseモード（-v）が正しく動作することをテスト
///
/// # テスト内容
///
/// - -vオプションを指定してYAMLテストを実行
/// - 詳細なレスポンス情報が表示されることを確認
///
/// # 検証項目
///
/// - コマンドが正常終了すること
/// - 標準出力に「Status Code: 200」が含まれること
/// - 詳細なレスポンス情報が表示されること
#[test]
fn test_yaml_file_verbose_mode() {
    // Arrange: Verboseモード用のYAMLファイルを作成
    let yaml_content = r#"
name: "Verbose Mode Test"
base_url: "https://jsonplaceholder.typicode.com"
tests:
  - name: "Verbose Test"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドの実行（verboseモード）
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap(), "-v"])
        .output()
        .expect("Failed to execute command");

    // Assert: 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Status Code: 200"));
}

/// YAMLファイルにbase_urlが指定されていない場合のエラー処理をテスト
///
/// # テスト内容
///
/// - base_urlフィールドが欠けているYAMLファイルを実行
/// - 適切なエラーメッセージが表示されることを確認
///
/// # 検証項目
///
/// - コマンドが異常終了すること
/// - 標準エラー出力に「ベースURLが指定されていません」が含まれること
#[test]
fn test_yaml_file_without_base_url() {
    // Arrange: base_urlが欠けているYAMLファイルを作成
    let yaml_content = r#"
name: "No Base URL Test"
tests:
  - name: "Test"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Assert: ベースURLがない場合はエラーになる
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("ベースURLが指定されていません"));
}

/// コマンドライン引数でbase_urlを上書きできることをテスト
///
/// # テスト内容
///
/// - YAMLファイルに定義されたbase_urlをCLI引数で上書き
/// - --base-urlオプションが優先されることを確認
///
/// # 検証項目
///
/// - コマンドが正常終了すること
/// - 上書きされたbase_url（jsonplaceholder）でリクエストが成功すること
/// - テストがPASSすること
#[test]
fn test_yaml_file_with_cli_base_url_override() {
    // Arrange: example.comをbase_urlとして定義（到達不可）
    let yaml_content = r#"
name: "Base URL Override Test"
base_url: "https://example.com"
tests:
  - name: "Override Test"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドライン引数でベースURLを上書き
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "-t",
            file.path().to_str().unwrap(),
            "--base-url",
            "https://jsonplaceholder.typicode.com",
        ])
        .output()
        .expect("Failed to execute command");

    // Assert: jsonplaceholder.typicode.comに向けてリクエストが送られるので成功する
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("✓ PASS Override Test"));
}

/// POSTメソッドのYAMLテストが正しく実行されることをテスト
///
/// # テスト内容
///
/// - POSTリクエストを含むYAMLテストファイルを実行
/// - JSON形式のリクエストボディを送信
/// - JSONPlaceholder APIに新規記事を作成
///
/// # 検証項目
///
/// - コマンドが正常終了すること
/// - テストがPASSすること
/// - ステータスコード201が正しく検証されること
/// - テスト結果サマリーが正しいこと（合計1、成功1、失敗0）
#[test]
fn test_yaml_file_post_request() {
    // Arrange: POSTリクエストを含むYAMLファイルを作成
    let yaml_content = r#"
name: "POST Method Test"
description: "Test POST requests"
base_url: "https://jsonplaceholder.typicode.com"
tests:
  - name: "Create Post"
    request:
      method: "POST"
      url: "/posts"
      headers:
        Content-Type: "application/json"
      body:
        title: "Test Title"
        body: "Test Content"
        userId: 1
    expectations:
      status_code: 201
"#;

    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", yaml_content).unwrap();

    // Act: コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Assert: 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success());
    assert!(stdout.contains("✓ PASS Create Post"));
    assert!(stdout.contains("合計: 1 / 成功: 1 / 失敗: 0"));
}
