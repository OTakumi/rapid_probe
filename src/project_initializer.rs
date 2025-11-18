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

    // プロジェクトディレクトリを作成（既存チェックも兼ねる）
    match fs::create_dir(&project_path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(anyhow!(
                "ディレクトリ '{}' は既に存在します。\n\n解決方法:\n  1. 別のディレクトリに移動: cd ../other_project\n  2. 既存ディレクトリを削除: rm -rf {} (注意: データが失われます)\n  3. 既存ディレクトリを使用: rapid_probe -t {}/tests/example.yaml",
                PROJECT_DIR, PROJECT_DIR, PROJECT_DIR
            ));
        }
        Err(e) => {
            return Err(e).context(format!(
                "ディレクトリ '{}' の作成に失敗しました",
                PROJECT_DIR
            ))
        }
    }

    println!("Rapid Probe プロジェクトを初期化しています...\n");
    println!("✓ ディレクトリを作成: {}/", PROJECT_DIR);

    // サブディレクトリとファイルの作成（失敗時はロールバック）
    if let Err(e) = create_subdirs_and_files(&project_path) {
        // クリーンアップ: 部分的に作成されたディレクトリ構造を削除
        let _ = fs::remove_dir_all(&project_path);
        return Err(e);
    }

    print_success_message();

    Ok(())
}

/// サブディレクトリとファイルの作成
///
/// testsディレクトリとテンプレートファイル（config.yaml, example.yaml）を作成します。
/// エラーが発生した場合、呼び出し側でクリーンアップが必要です。
fn create_subdirs_and_files(project_path: &Path) -> Result<()> {
    // testsディレクトリの作成
    let tests_path = project_path.join(TESTS_DIR);
    fs::create_dir(&tests_path).with_context(|| {
        format!(
            "ディレクトリ '{}/{}' の作成に失敗しました",
            PROJECT_DIR, TESTS_DIR
        )
    })?;
    println!("✓ ディレクトリを作成: {}/{}/", PROJECT_DIR, TESTS_DIR);

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
        // テンプレートファイルが埋め込まれていること（最小サイズをチェック）
        assert!(
            CONFIG_TEMPLATE.len() > 100,
            "Config template seems too short: {} bytes",
            CONFIG_TEMPLATE.len()
        );
        assert!(
            EXAMPLE_TEST_TEMPLATE.len() > 100,
            "Example template seems too short: {} bytes",
            EXAMPLE_TEST_TEMPLATE.len()
        );

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

    #[test]
    fn test_created_files_contain_correct_content() {
        // Arrange: 一時ディレクトリを作成
        let temp_dir = TempDir::new().unwrap();

        // Act: プロジェクトを初期化
        initialize_project_at(temp_dir.path()).unwrap();

        // Assert: ファイルの内容が正しいこと
        let project_path = temp_dir.path().join(PROJECT_DIR);

        // config.yamlの内容を検証
        let config_content = fs::read_to_string(project_path.join(CONFIG_FILE)).unwrap();
        assert_eq!(
            config_content, CONFIG_TEMPLATE,
            "config.yaml content should match template"
        );

        // example.yamlの内容を検証
        let example_content =
            fs::read_to_string(project_path.join(TESTS_DIR).join(EXAMPLE_TEST_FILE)).unwrap();
        assert_eq!(
            example_content, EXAMPLE_TEST_TEMPLATE,
            "example.yaml content should match template"
        );
    }

    #[test]
    fn test_initialize_with_unicode_base_path() {
        // Arrange: Unicode文字（日本語）を含むパスを作成
        let temp_dir = TempDir::new().unwrap();
        let unicode_path = temp_dir.path().join("テスト日本語");
        fs::create_dir(&unicode_path).unwrap();

        // Act: プロジェクトを初期化
        let result = initialize_project_at(&unicode_path);

        // Assert: 成功すること
        assert!(result.is_ok(), "Should handle Unicode paths correctly");

        let project_path = unicode_path.join(PROJECT_DIR);
        assert!(project_path.exists());
        assert!(project_path.join(TESTS_DIR).exists());
        assert!(project_path.join(CONFIG_FILE).exists());
    }

    #[test]
    fn test_initialize_with_spaces_in_path() {
        // Arrange: スペースを含むパスを作成
        let temp_dir = TempDir::new().unwrap();
        let space_path = temp_dir.path().join("my projects");
        fs::create_dir(&space_path).unwrap();

        // Act: プロジェクトを初期化
        let result = initialize_project_at(&space_path);

        // Assert: 成功すること
        assert!(result.is_ok(), "Should handle paths with spaces correctly");

        let project_path = space_path.join(PROJECT_DIR);
        assert!(project_path.exists());
        assert!(project_path.join(TESTS_DIR).exists());
        assert!(project_path.join(CONFIG_FILE).exists());
    }
}
