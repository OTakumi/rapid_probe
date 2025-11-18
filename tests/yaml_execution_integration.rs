use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_yaml_file_execution() {
    // YAMLテストファイルの作成
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

    // コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success());
    assert!(stdout.contains("テストケースファイルを読み込み中"));
    assert!(stdout.contains("✓ PASS Get Request Test"));
    assert!(stdout.contains("合計: 1 / 成功: 1 / 失敗: 0"));
}

#[test]
fn test_yaml_file_with_failing_test() {
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

    // コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);

    // 失敗したテストがある場合、exitコードは1
    assert!(!output.status.success());
    assert!(stdout.contains("✓ PASS Expected 200 Test"));
    assert!(stdout.contains("✗ FAIL Expected 200 but get 404"));
    assert!(stdout.contains("合計: 2 / 成功: 1 / 失敗: 1"));
}

#[test]
fn test_yaml_file_verbose_mode() {
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

    // コマンドの実行（verboseモード）
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap(), "-v"])
        .output()
        .expect("Failed to execute command");

    // 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("Status Code: 200"));
}

#[test]
fn test_yaml_file_without_base_url() {
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

    // コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // ベースURLがない場合はエラーになる
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success());
    assert!(stderr.contains("ベースURLが指定されていません"));
}

#[test]
fn test_yaml_file_with_cli_base_url_override() {
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

    // コマンドライン引数でベースURLを上書き
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

    // jsonplaceholder.typicode.comに向けてリクエストが送られるので成功する
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("✓ PASS Override Test"));
}

#[test]
fn test_yaml_file_post_request() {
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

    // コマンドの実行
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", file.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // 結果の検証
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(output.status.success());
    assert!(stdout.contains("✓ PASS Create Post"));
    assert!(stdout.contains("合計: 1 / 成功: 1 / 失敗: 0"));
}
