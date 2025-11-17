use anyhow::{Context, Result, anyhow};
use clap::Parser;
use rapid_probe::{ApiClient, HttpClient, TestCaseLoader, TestRunner};
use std::collections::HashMap;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Rapid Probe")]
#[command(about = "汎用APIテストランナー")]
struct Cli {
    /// URL
    url: Option<String>,

    /// HTTPメソッド
    #[arg(short = 'X', long = "request")]
    request: Option<String>,

    /// HTTPヘッダー
    #[arg(short = 'H', long = "header")]
    headers: Vec<String>,

    /// リクエストデータ
    #[arg(short = 'd', long = "data")]
    data: Option<String>,

    /// 詳細出力
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// ヘッダーを含めて出力
    #[arg(short = 'i', long = "include")]
    include: bool,

    /// サイレントモード
    #[arg(short = 's', long = "silent")]
    silent: bool,

    /// テストケースファイル
    #[arg(short = 't', long = "test-case")]
    test_case: Option<String>,

    /// ベースURL
    #[arg(long = "base-url")]
    base_url: Option<String>,

    /// 認証トークン
    #[arg(long = "token")]
    token: Option<String>,

    /// レポート形式
    #[arg(long = "report-format", default_value = "console")]
    report_format: String,
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
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // サイレントモードでない場合のみ表示
    if !cli.silent {
        println!("Rapid Probe - 汎用APIテストランナー");
    }

    // 実行モードの判定
    match (&cli.url, &cli.test_case) {
        // 単一URLリクエストモード
        (Some(url), None) => {
            execute_single_request(&cli, url).await?;
        }
        // テストケースモード
        (None, Some(test_case)) => {
            execute_test_case_file(&cli, test_case).await?;
        }
        // 両方指定された場合
        (Some(_), Some(_)) => {
            return Err(anyhow!(
                "URLとテストケースファイルの両方を同時に指定することはできません"
            ));
        }
        // どちらも指定されていない場合
        (None, None) => {
            return Err(anyhow!(
                "URLまたはテストケースファイルを指定してください\n使用方法: rapid_probe <URL> または rapid_probe -t <test-case-file>"
            ));
        }
    }

    Ok(())
}

async fn execute_single_request(cli: &Cli, url: &str) -> Result<()> {
    // URLの解析
    let (base_url, path) = match url {
        // フルURLが指定された場合
        url if url.starts_with("http://") || url.starts_with("https://") => {
            let parsed = url::Url::parse(url).with_context(|| format!("invalid URL: {}", url))?;
            let host = parsed
                .host_str()
                .ok_or_else(|| anyhow!("URL has no host: {}", url))?;
            let base = format!("{}://{}", parsed.scheme(), host);
            let path = parsed.path().to_string()
                + &parsed
                    .query()
                    .map(|q| format!("?{}", q))
                    .unwrap_or_default();
            (base, path)
        }
        // 相対パスが指定された場合
        relative_path => match &cli.base_url {
            Some(base) => (base.clone(), relative_path.to_string()),
            None => {
                return Err(anyhow!(
                    "相対URLが指定されましたが、--base-urlが設定されていません"
                ));
            }
        },
    };

    // APIクライアントの作成
    let client = ApiClient::new(&base_url).context("failed to create API client")?;

    // ヘッダーの準備
    let mut headers = HashMap::new();

    // トークンの追加
    if let Some(token) = &cli.token {
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
    }

    // カスタムヘッダーの追加
    for header in &cli.headers {
        if let Some((key, value)) = header.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // 詳細モードでの出力
    if cli.verbose {
        eprintln!("* Connecting to {}", base_url);
        eprintln!(
            "> {} {} HTTP/1.1",
            cli.request.as_deref().unwrap_or("GET"),
            path
        );
        for (k, v) in &headers {
            eprintln!("> {}: {}", k, v);
        }
    }

    // HTTPメソッドに応じたリクエストの実行
    let method = cli.request.as_deref().unwrap_or("GET");

    let (status, body) = match method.to_uppercase().as_str() {
        "GET" => client
            .get_with_headers(&path, headers)
            .await
            .context("HTTP GET request failed")?,
        "POST" => {
            return Err(anyhow!("HTTPメソッド 'POST' はまだサポートされていません"));
        }
        "PUT" => {
            return Err(anyhow!("HTTPメソッド 'PUT' はまだサポートされていません"));
        }
        "DELETE" => {
            return Err(anyhow!(
                "HTTPメソッド 'DELETE' はまだサポートされていません"
            ));
        }
        "PATCH" => {
            return Err(anyhow!("HTTPメソッド 'PATCH' はまだサポートされていません"));
        }
        _ => {
            return Err(anyhow!("不明なHTTPメソッド: '{}'", method));
        }
    };

    // レスポンスの表示
    if cli.verbose {
        eprintln!("< HTTP/1.1 {}", status);
    }

    if cli.include {
        println!("HTTP/1.1 {}", status);
        println!();
    }

    if !cli.silent {
        println!("{}", body);
    }

    Ok(())
}

async fn execute_test_case_file(cli: &Cli, test_case_file: &str) -> Result<()> {
    if !cli.silent {
        println!("テストケースファイルを読み込み中: {}", test_case_file);
    }

    // テストケースファイルのロード
    let test_suite = TestCaseLoader::load_from_file(Path::new(test_case_file))
        .with_context(|| format!("failed to load test file: {}", test_case_file))?;

    // ベースURLの決定（コマンドライン引数 > YAMLファイル）
    let base_url = cli.base_url.as_ref()
        .or(test_suite.base_url.as_ref())
        .ok_or_else(|| anyhow!("ベースURLが指定されていません。--base-url オプションまたはYAMLファイルで指定してください。"))?;

    // APIクライアントの作成
    let client = ApiClient::new(base_url).context("failed to create API client")?;

    // テストランナーの作成と実行
    let runner = TestRunner::new(client);
    let results = runner.run_test_suite(&test_suite, Some(base_url)).await;

    // 結果のレポート
    if !cli.silent {
        println!("\n=== テスト結果 ===");
        if let Some(name) = &test_suite.name {
            println!("テストスイート: {}", name);
        }
        if let Some(desc) = &test_suite.description {
            println!("説明: {}", desc);
        }
        println!();

        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        for result in &results {
            let status = if result.passed {
                "✓ PASS"
            } else {
                "✗ FAIL"
            };
            println!(
                "{} {} ({}ms)",
                status,
                result.test_name,
                result.duration.as_millis()
            );

            if cli.verbose {
                // ステータスコードを表示
                if let Some(status_code) = result.status_code {
                    println!("  Status Code: {}", status_code);
                }

                // アサーション結果を表示
                for assertion in &result.assertions {
                    if !assertion.passed {
                        println!(
                            "  {} - Expected: {}, Actual: {}",
                            assertion.assertion_type, assertion.expected, assertion.actual
                        );
                        if let Some(msg) = &assertion.message {
                            println!("    {}", msg);
                        }
                    }
                }

                // エラーメッセージを表示
                if let Some(err) = &result.error_message {
                    println!("  Error: {}", err);
                }
            }
        }

        println!("\n合計: {} / 成功: {} / 失敗: {}", total, passed, failed);

        // 失敗があった場合は非ゼロの終了コード
        if failed > 0 {
            std::process::exit(1);
        }
    }

    Ok(())
}
