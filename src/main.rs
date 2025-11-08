use clap::Parser;
use rapid_probe::{ApiClient, HttpClient};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "Rapid Probe")]
#[command(about = "汎用APIテストランナー")]
struct Cli {
    /// テストケースファイル
    #[arg(short, long)]
    test_case: Option<String>,

    /// ベースURL
    #[arg(long)]
    base_url: Option<String>,

    /// 認証トークン
    #[arg(long)]
    token: Option<String>,

    /// レポート形式 (json)
    #[arg(long, default_value = "console")]
    report_format: String,

    #[arg(short, long)]
    verbose: bool,
}

// #[derive(Deserialize)]
// struct TestCase {
//     name: String,
//     description: Option<String>,
//     request: Request,
//     expectations: Expectations,
//     variables: Option<HashMap<String, String>>,
// }
//
// #[derive(Deserialize)]
// struct Request {
//     method: String,
//     url: String,
//     headers: Option<HashMap<String, String>>,
//     body_file: Option<String>,
//     body: Option<serde_json::Value>,
// }
//
// #[derive(Deserialize)]
// struct Expectations {
//     status_code: u16,
//     headers: Option<HashMap<String, String>>,
//     response: Option<ResponseExpectation>,
//     performance: Option<PerformanceExpectation>,
//     custom_assertions: Option<Vec<CustomAssertion>>,
// }
//
// #[derive(Deserialize)]
// struct ResponseExpectation {
//     schema_file: Option<String>,
//     expected_file: Option<String>,
// }
//
// #[derive(Deserialize)]
// struct PerformanceExpectation {
//     max_response_time_ms: u64,
// }
//
// #[derive(Deserialize)]
// struct CustomAssertion {
//     field: String,
//     equals: serde_json::Value,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Rapid Probe - 汎用APIテストランナー");

    if cli.verbose {
        println!("詳細モード: 有効");
    }

    if let Some(test_case) = &cli.test_case {
        println!("テストケースファイル: {}", test_case);
    }

    if let Some(base_url) = &cli.base_url {
        println!("ベースURL: {}", base_url);
    }

    println!("レポート形式: {}", cli.report_format);

    // TODO: テストランナーの実装
    println!("\nテスト実行機能は現在開発中です。");

    Ok(())
}
