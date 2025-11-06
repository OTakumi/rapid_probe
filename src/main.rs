// use clap::Parser;
// use rapid_probe::{ApiClient, HttpClient};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
//
// #[derive(Parser)]
// #[command(name = "Rapid Probe")]
// #[command(about = "汎用APIテストランナー")]
// struct Cli {
//     /// テストケースファイル
//     #[arg(short, long)]
//     test_case: Option<String>,
//
//     /// ベースURL
//     #[arg(long)]
//     base_url: Option<String>,
//
//     /// 認証トークン
//     #[arg(long)]
//     token: Option<String>,
//
//     /// レポート形式 (json)
//     #[arg(long, default_value = "console")]
//     report_format: String,
//
//     #[arg(short, long)]
//     verbose: bool,
// }
//
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
async fn main() {
    // let cli = Cli::parse();
    //
    // let runner = ApiTestRunner::new(&cli).await?;
    // let results = runner.run().await?;
    //
    // runner.generate_report(&results, &cli.report_format).await?;
    //
    // // 失敗テストがある場合は非ゼロで終了
    // if results.iter().any(|r| !r.passed) {
    //     std::process::exit(1);
    // }
    //
    // Ok(())
    print!("Main");
}
