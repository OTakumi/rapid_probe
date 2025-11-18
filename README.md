# Rapid Probe

## 概要

Rapid Probeは、APIのE2Eテストを自動化するための汎用的なテストツールです。<br />
curlライクなインターフェースを提供し、単一のAPIリクエストから複雑なテストシナリオまで対応できます。

## インストール

### 必要な環境

- Rust 1.70以上

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/rapid_probe.git
cd rapid_probe

# リリースビルド
cargo build --release

# バイナリを確認
./target/release/rapid_probe --help
```

### パスを通す（オプション）

ビルドしたバイナリを任意の場所で使用できるようにします：

```bash
# macOS / Linux
sudo cp target/release/rapid_probe /usr/local/bin/

# または、ホームディレクトリの bin/ に配置
mkdir -p ~/bin
cp target/release/rapid_probe ~/bin/
# ~/.bashrc または ~/.zshrc に以下を追加
# export PATH="$HOME/bin:$PATH"

# Windows (PowerShell)
# C:\Program Files\rapid_probe\ などに配置し、環境変数 PATH に追加
```

### 動作確認

```bash
rapid_probe --help
rapid_probe https://jsonplaceholder.typicode.com/posts/1
```

## 使い方

### プロジェクトの初期化

`rapid_probe init`コマンドを使用して、テストプロジェクトの雛形を作成できます。

```bash
rapid_probe init
```

このコマンドを実行すると、カレントディレクトリに以下の構造が作成されます：

```
rapid_probe/
├── config.yaml          # プロジェクト設定ファイル
└── tests/
    └── example.yaml     # サンプルテストケース
```

**生成されるファイル**:

- **`config.yaml`**: ベースURL、認証情報、タイムアウト設定などのプロジェクト全体の設定
- **`tests/example.yaml`**: GET/POSTリクエストのサンプルテストケース（すぐに実行可能）

**次のステップ**:

1. `config.yaml`を編集して、テスト対象のAPIのベースURLを設定
2. `tests/example.yaml`を参考に、独自のテストケースを作成
3. テストを実行:
   ```bash
   rapid_probe -t rapid_probe/tests/example.yaml
   ```

### 基本的な使用方法

```bash
# シンプルなGETリクエスト
rapid_probe https://api.example.com/users

# POSTリクエストでJSONデータを送信
rapid_probe https://api.example.com/users -X POST -d '{"name": "John", "email": "john@example.com"}'

# 詳細出力モード
rapid_probe https://api.example.com/users -v

# カスタムヘッダーを追加
rapid_probe https://api.example.com/users -H "X-API-Key: your-key"

# 認証トークンを使用
rapid_probe https://api.example.com/users --token "your-bearer-token"

# レスポンスヘッダーを含めて表示
rapid_probe https://api.example.com/users -i
```

### YAMLテストケースの使用方法

複数のテストケースをYAMLファイルで定義して実行できます。

```yaml
# test.yaml
name: "API Test Suite"
description: "Example test cases"
base_url: "https://jsonplaceholder.typicode.com"

tests:
  - name: "Get Single Post"
    request:
      method: "GET"
      url: "/posts/1"
    expectations:
      status_code: 200

  - name: "Create New Post"
    request:
      method: "POST"
      url: "/posts"
      headers:
        Content-Type: "application/json"
      body:
        title: "Test Post"
        body: "This is a test"
        userId: 1
    expectations:
      status_code: 201
```

実行：

```bash
rapid_probe -t test.yaml

# 詳細モードで実行
rapid_probe -t test.yaml -v

# ベースURLを上書き
rapid_probe -t test.yaml --base-url https://api.example.com
```

### コマンドラインオプション

curlと互換性のあるオプション:

- `-X, --request <METHOD>`: HTTPメソッドを指定（GET、POSTをサポート）
- `-H, --header <HEADER>`: カスタムHTTPヘッダーを追加
- `-d, --data <DATA>`: リクエストボディデータを送信
- `-v, --verbose`: 詳細な出力（リクエスト/レスポンスの詳細）
- `-i, --include`: レスポンスヘッダーを含めて表示
- `-s, --silent`: サイレントモード（余分な出力を抑制）

独自オプション:

- `-t, --test-case <FILE>`: YAMLテストケースファイルを指定
- `--base-url <URL>`: ベースURLを設定（YAMLファイル内の定義を上書き）
- `--token <TOKEN>`: Bearer認証トークンを設定
- `--report-format <FORMAT>`: レポート形式を指定（デフォルト: console）

サブコマンド:

- `init`: テストプロジェクトの雛形を作成（詳細は「プロジェクトの初期化」セクションを参照）

## 現在実装されている機能

### HTTPリクエスト機能

- ✅ **GETリクエストの送信**: 標準的なGETリクエストでリソースを取得できます
- ✅ **POSTリクエストの送信**: JSONボディを含むPOSTリクエストでデータを送信できます
- ✅ **カスタムHTTPヘッダーの追加**: `-H`オプションで任意のヘッダーを追加できます
- ✅ **Bearer認証トークンの設定**: `--token`オプションで簡単に認証トークンを設定できます
- ✅ **リクエストボディの送信**: `-d`オプションでJSONデータやテキストを送信できます

### レスポンス表示機能

- ✅ **レスポンスボディの表示**: APIからのレスポンスを標準出力に表示します
- ✅ **レスポンスヘッダーの表示**: `-i`オプションでレスポンスヘッダーを含めて表示できます
- ✅ **詳細モード**: `-v`オプションでリクエスト/レスポンスの詳細情報を表示します
- ✅ **サイレントモード**: `-s`オプションで余分な出力を抑制できます

### テスト機能

- ✅ **YAMLベースのテストケース定義**: YAMLファイルで複数のテストケースを定義できます
- ✅ **レスポンスの自動検証**: ステータスコードの期待値を定義して自動検証できます
- ✅ **複数のテストケースの連続実行**: 1つのYAMLファイルに複数のテストを定義して順次実行できます
- ✅ **テスト結果のサマリー表示**: 成功/失敗の件数とテスト結果を見やすく表示します
- ✅ **変数の定義**: YAMLファイルで`base_url`を定義し、コマンドラインで上書きできます

### プロジェクト管理機能

- ✅ **プロジェクト初期化**: `rapid_probe init`コマンドでテストプロジェクトの雛形を自動生成できます
  - `config.yaml`: プロジェクト設定ファイル（ベースURL、認証、タイムアウト、並列実行設定）
  - `tests/example.yaml`: すぐに実行できるサンプルテストケース（GET/POSTの例を含む）
  - エラー時の自動ロールバック: 初期化中にエラーが発生した場合、部分的に作成されたファイルを自動削除
  - Unicode/スペース対応: 日本語やスペースを含むパスでも正常に動作

### アーキテクチャ・パターン

- ✅ **Strategy Pattern**: HTTPメソッドごとにStrategyパターンを適用し、拡張性を確保しています
- ✅ **Factory Pattern**: `StrategyFactory`でHTTPメソッドに応じた適切なStrategyを生成します
- ✅ **curlライクなインターフェース**: curl使用者にとって馴染みやすいコマンド体系を提供します

### セキュリティ機能

- ✅ **包括的なSSRF対策**: プライベートIP、localhost、link-localアドレスへのアクセスをブロックします
  - IPv4: `127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `169.254.0.0/16`
  - IPv6: `::1`, `fc00::/7`, `fe80::/10`
  - 特殊アドレス: `0.0.0.0/8`, `::/128`, `255.255.255.255`
- ✅ **タイムアウト設定**: 接続タイムアウト（10秒）と全体タイムアウト（30秒）を設定済み

### 品質保証

- ✅ **包括的なテストカバレッジ**: 84個のテスト（単体テスト69個、統合テスト8個、E2Eテスト6個、Docテスト1個）
- ✅ **統合テスト**: 実際の外部API（JSONPlaceholder）を使用した統合テストを実装
- ✅ **E2Eテスト**: cargo runサブプロセスを使用したフルE2Eテストを実装
- ✅ **エッジケーステスト**: Unicode文字、スペースを含むパス、ファイル内容検証などを含む

## 今後実装予定の機能

### HTTPメソッドの拡張

- [ ] **PUT、DELETE、PATCHメソッドのサポート**: RESTful APIの完全なCRUD操作に対応します
- [ ] **ファイルからのリクエストボディ読み込み**: `@file.json`形式でファイルからボディを読み込めるようにします
- [ ] **マルチパートフォームデータ**: ファイルアップロードに対応します

### テスト機能の拡張

- [ ] **JSONスキーマによるレスポンス検証**: レスポンスボディの構造を厳密に検証できるようにします
- [ ] **レスポンスボディの検証**: 特定のフィールドの値や存在をアサートできるようにします
- [ ] **レスポンスヘッダーの検証**: 期待されるヘッダーが含まれているかチェックできるようにします
- [ ] **環境変数のサポート**: テスト間で値を共有し、前のレスポンスから値を抽出できるようにします
- [ ] **テストの依存関係**: 前のテストの結果を次のテストで使用できるようにします

### レポート機能

- [ ] **テスト結果のJSON形式出力**: 機械可読な形式でテスト結果を出力します
- [ ] **HTMLレポートの生成**: ブラウザで見やすいテストレポートを生成します
- [ ] **JUnitフォーマット対応**: CI/CDツールとの統合を容易にします
- [ ] **実行時間の測定とパフォーマンス統計**: 各テストの実行時間とパフォーマンスを記録します

### その他の機能

- [ ] **リトライ機能**: 失敗したリクエストを自動的にリトライします
- [ ] **カスタムタイムアウト設定**: ユーザーがタイムアウト値を指定できるようにします
- [ ] **プロキシサポート**: HTTPプロキシ経由でリクエストを送信できるようにします
- [ ] **証明書の検証設定**: 自己署名証明書の許可など、SSL証明書の検証を制御できるようにします
- [ ] **レスポンスの保存機能**: レスポンスをファイルに保存できるようにします
- [ ] **環境変数からの設定読み込み**: `.env`ファイルや環境変数から設定を読み込めるようにします

## 制限事項

セキュリティ上の理由から、以下のアドレスへのアクセスはブロックされます：

- **localhost**: `127.0.0.1`、`localhost`など
- **プライベートIP**: `10.x.x.x`、`192.168.x.x`、`172.16-31.x.x`
- **Link-localアドレス**: `169.254.x.x`など

これらのアドレスへリクエストを送信しようとすると、エラーが返されます。

**開発・テスト環境での対処法**

ローカル環境のAPIをテストする場合は、以下の方法があります：

1. **ポートフォワーディング**: localhostではなく、パブリックIPやドメイン経由でアクセス
2. **ngrokなどのトンネリングツール**: ローカルサーバーを一時的に公開URL経由でアクセス
3. **テストコード内での使用**: Rust統合テストでは`ApiClient::new_for_testing()`を使用可能

## 開発

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# Lintチェック
cargo clippy

# フォーマット
cargo fmt

# 開発モードで実行
cargo run -- https://jsonplaceholder.typicode.com/posts/1

# YAMLテストケースで実行
cargo run -- -t examples/simple_api_test.yaml -v
```
