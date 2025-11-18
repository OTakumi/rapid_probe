//! Init コマンドのエントリーポイント
//!
//! `rapid_probe init`コマンドの実行時に呼び出されます。

use anyhow::Result;

use crate::project_initializer;

/// initコマンドを実行
///
/// プロジェクトディレクトリと設定ファイルを初期化します。
pub fn execute() -> Result<()> {
    project_initializer::initialize_project()
}
