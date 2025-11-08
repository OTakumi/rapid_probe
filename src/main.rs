use clap::Parser;
use rapid_probe::{ApiClient, HttpClient};
use std::collections::HashMap;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            println!("テストケースファイル: {}", test_case);
            println!("テスト実行機能は現在開発中です。");
        }
        // 両方指定された場合
        (Some(_), Some(_)) => {
            eprintln!("エラー: URLとテストケースファイルの両方を同時に指定することはできません。");
            std::process::exit(1);
        }
        // どちらも指定されていない場合
        (None, None) => {
            eprintln!("URLまたはテストケースファイルを指定してください。");
            eprintln!("使用方法: rapid_probe <URL> または rapid_probe -t <test-case-file>");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn execute_single_request(cli: &Cli, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // URLの解析
    let (base_url, path) = match url {
        // フルURLが指定された場合
        url if url.starts_with("http://") || url.starts_with("https://") => {
            let parsed = url::Url::parse(url)?;
            let base = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or(""));
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
                return Err("相対URLが指定されましたが、--base-urlが設定されていません".into());
            }
        },
    };

    // APIクライアントの作成
    let client = ApiClient::new(&base_url)?;

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
        "GET" => client.get_with_headers(&path, headers).await?,
        "POST" => {
            return Err(format!("HTTPメソッド 'POST' はまだサポートされていません").into());
        }
        "PUT" => {
            return Err(format!("HTTPメソッド 'PUT' はまだサポートされていません").into());
        }
        "DELETE" => {
            return Err(format!("HTTPメソッド 'DELETE' はまだサポートされていません").into());
        }
        "PATCH" => {
            return Err(format!("HTTPメソッド 'PATCH' はまだサポートされていません").into());
        }
        _ => {
            return Err(format!("不明なHTTPメソッド: '{}'", method).into());
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
