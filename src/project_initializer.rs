//! プロジェクト初期化処理
//!
//! ディレクトリ構造の作成とテンプレートファイルの配置を行います。

use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

const PROJECT_DIR: &str = "rapid_probe";
const TESTS_DIR: &str = "tests";
const CONFIG_FILE: &str = "config.yaml";
const EXAMPLE_TEST_FILE: &str = "example.yaml";

// テンプレートファイルをバイナリに埋め込む
const CONFIG_TEMPLATE: &str = include_str!("../templates/config.yaml");
const EXAMPLE_TEST_TEMPLATE: &str = include_str!("../templates/example.yaml");

/// プロジェクトディレクトリを初期化（カレントディレクトリ）
///
/// カレントディレクトリに`rapid_probe`ディレクトリを作成し、
/// 必要なファイルとディレクトリを生成します。
///
/// # Returns
///
/// 成功した場合は`Ok(())`、失敗した場合はエラーを返します。
///
/// # Errors
///
/// - ディレクトリが既に存在する場合
/// - ファイルの作成に失敗した場合
pub fn initialize_project() -> Result<()> {
    let current_dir =
        std::env::current_dir().context("カレントディレクトリの取得に失敗しました")?;
    initialize_project_at(&current_dir)
}

/// プロジェクトディレクトリを初期化（指定されたパス）
///
/// 指定されたベースパスに`rapid_probe`ディレクトリを作成し、
/// 必要なファイルとディレクトリを生成します。
///
/// # Arguments
///
/// * `base_path` - プロジェクトディレクトリを作成する親ディレクトリのパス
///
/// # Returns
///
/// 成功した場合は`Ok(())`、失敗した場合はエラーを返します。
///
/// # Errors
///
/// - ディレクトリが既に存在する場合
/// - ファイルの作成に失敗した場合
pub fn initialize_project_at(base_path: &Path) -> Result<()> {
    let project_path = base_path.join(PROJECT_DIR);

    // ディレクトリが既に存在する場合はエラー
    validate_project_directory(&project_path)?;

    println!("Rapid Probe プロジェクトを初期化しています...\n");

    // プロジェクト構造の作成
    create_directories(&project_path)?;
    create_files(&project_path)?;

    print_success_message();

    Ok(())
}

/// プロジェクトディレクトリの存在チェック
fn validate_project_directory(project_path: &Path) -> Result<()> {
    if project_path.exists() {
        return Err(anyhow!(
            "ディレクトリ '{}' は既に存在します。\n別のディレクトリで実行するか、既存のディレクトリを削除してください。",
            PROJECT_DIR
        ));
    }
    Ok(())
}

/// ディレクトリ構造の作成
fn create_directories(project_path: &Path) -> Result<()> {
    // プロジェクトディレクトリの作成
    fs::create_dir(project_path)
        .with_context(|| format!("ディレクトリ '{}' の作成に失敗しました", PROJECT_DIR))?;
    println!("✓ ディレクトリを作成: {}/", PROJECT_DIR);

    // testsディレクトリの作成
    let tests_path = project_path.join(TESTS_DIR);
    fs::create_dir(&tests_path).with_context(|| {
        format!(
            "ディレクトリ '{}/{}' の作成に失敗しました",
            PROJECT_DIR, TESTS_DIR
        )
    })?;
    println!("✓ ディレクトリを作成: {}/{}/", PROJECT_DIR, TESTS_DIR);

    Ok(())
}

/// テンプレートファイルの作成
fn create_files(project_path: &Path) -> Result<()> {
    // config.yamlの作成
    create_config_file(project_path)?;

    // example.yamlの作成
    create_example_test_file(project_path)?;

    Ok(())
}

/// config.yamlの作成
fn create_config_file(project_path: &Path) -> Result<()> {
    let config_path = project_path.join(CONFIG_FILE);
    fs::write(&config_path, CONFIG_TEMPLATE).with_context(|| {
        format!(
            "ファイル '{}/{}' の作成に失敗しました",
            PROJECT_DIR, CONFIG_FILE
        )
    })?;
    println!("✓ ファイルを作成: {}/{}", PROJECT_DIR, CONFIG_FILE);
    Ok(())
}

/// example.yamlの作成
fn create_example_test_file(project_path: &Path) -> Result<()> {
    let tests_path = project_path.join(TESTS_DIR);
    let example_path = tests_path.join(EXAMPLE_TEST_FILE);
    fs::write(&example_path, EXAMPLE_TEST_TEMPLATE).with_context(|| {
        format!(
            "ファイル '{}/{}/{}' の作成に失敗しました",
            PROJECT_DIR, TESTS_DIR, EXAMPLE_TEST_FILE
        )
    })?;
    println!(
        "✓ ファイルを作成: {}/{}/{}",
        PROJECT_DIR, TESTS_DIR, EXAMPLE_TEST_FILE
    );
    Ok(())
}

/// 成功メッセージの表示
fn print_success_message() {
    println!("\n初期化が完了しました！\n");
    println!("次のステップ:");
    println!(
        "  1. {}/{}を編集して、テスト対象のAPIを設定してください",
        PROJECT_DIR, CONFIG_FILE
    );
    println!(
        "  2. {}/{}/{}を参考に、テストケースを作成してください",
        PROJECT_DIR, TESTS_DIR, EXAMPLE_TEST_FILE
    );
    println!(
        "  3. テストを実行: rapid_probe -t {}/{}/{}",
        PROJECT_DIR, TESTS_DIR, EXAMPLE_TEST_FILE
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_initialize_project_creates_directory_structure() {
        // Arrange: 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();

        // Act: プロジェクトを初期化
        let result = initialize_project_at(temp_dir.path());

        // Assert: 成功すること
        assert!(result.is_ok());

        let project_path = temp_dir.path().join(PROJECT_DIR);

        // ディレクトリが作成されていること
        assert!(project_path.exists());
        assert!(project_path.join(TESTS_DIR).exists());

        // ファイルが作成されていること
        assert!(project_path.join(CONFIG_FILE).exists());
        assert!(project_path
            .join(TESTS_DIR)
            .join(EXAMPLE_TEST_FILE)
            .exists());
    }

    #[test]
    fn test_initialize_project_fails_if_directory_exists() {
        // Arrange: 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();

        // 既にプロジェクトディレクトリを作成
        let project_path = temp_dir.path().join(PROJECT_DIR);
        fs::create_dir(&project_path).unwrap();

        // Act: プロジェクトを初期化（既に存在するディレクトリに対して）
        let result = initialize_project_at(temp_dir.path());

        // Assert: エラーが返されること
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("既に存在します"));
    }

    #[test]
    fn test_template_files_are_embedded() {
        // テンプレートファイルが埋め込まれていること
        assert!(!CONFIG_TEMPLATE.is_empty());
        assert!(!EXAMPLE_TEST_TEMPLATE.is_empty());

        // テンプレートに期待される内容が含まれていること
        assert!(CONFIG_TEMPLATE.contains("base_url"));
        assert!(CONFIG_TEMPLATE.contains("auth"));
        assert!(CONFIG_TEMPLATE.contains("timeout"));
        assert!(CONFIG_TEMPLATE.contains("parallel"));

        assert!(EXAMPLE_TEST_TEMPLATE.contains("name:"));
        assert!(EXAMPLE_TEST_TEMPLATE.contains("tests:"));
        assert!(EXAMPLE_TEST_TEMPLATE.contains("GET"));
        assert!(EXAMPLE_TEST_TEMPLATE.contains("POST"));
    }
}
