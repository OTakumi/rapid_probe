# Rapid Probe

## 概要

Rapid Probeは、APIのE2Eテストを自動化するための汎用的なテストツールです。<br />
curlライクなインターフェースを提供し、単一のAPIリクエストから複雑なテストシナリオまで対応できます。

## 使い方

### 基本的な使用方法

```bash
# シンプルなGETリクエスト
rapid_probe https://api.example.com/users

# 詳細出力モード
rapid_probe https://api.example.com/users -v

# カスタムヘッダーを追加
rapid_probe https://api.example.com/users -H "X-API-Key: your-key"

# 認証トークンを使用
rapid_probe https://api.example.com/users --token "your-bearer-token"

# レスポンスヘッダーを含めて表示
rapid_probe https://api.example.com/users -i
```

### コマンドラインオプション

curlと互換性のあるオプション:

- `-X, --request <METHOD>`: HTTPメソッドを指定（現在はGETのみサポート）
- `-H, --header <HEADER>`: カスタムHTTPヘッダーを追加
- `-d, --data <DATA>`: リクエストボディデータ（未実装）
- `-v, --verbose`: 詳細な出力（リクエスト/レスポンスの詳細）
- `-i, --include`: レスポンスヘッダーを含めて表示
- `-s, --silent`: サイレントモード（余分な出力を抑制）

独自オプション:

- `-t, --test-case <FILE>`: テストケースファイルを指定
- `--base-url <URL>`: ベースURLを設定
- `--token <TOKEN>`: Bearer認証トークンを設定
- `--report-format <FORMAT>`: レポート形式を指定（デフォルト: console）

## 現在実装されている機能

- ✅ GETリクエストの送信
- ✅ レスポンスボディの表示
- ✅ カスタムHTTPヘッダーの追加
- ✅ Bearer認証トークンの設定
- ✅ 詳細モード（リクエスト/レスポンスの詳細表示）
- ✅ レスポンスヘッダーの表示
- ✅ サイレントモード
- ✅ curlライクなコマンドラインインターフェース

## 今後実装予定の機能

### HTTPメソッドの拡張

- [ ] POST、PUT、DELETE、PATCHメソッドのサポート
- [ ] リクエストボディの送信（-d オプション）
- [ ] JSONデータの自動整形とContent-Typeヘッダーの設定
- [ ] ファイルからのリクエストボディ読み込み

### テスト機能

- [ ] YAMLベースのテストケース定義
- [ ] レスポンスの自動検証（ステータスコード、ボディ、ヘッダー）
- [ ] JSONスキーマによるレスポンス検証
- [ ] 複数のテストケースの連続実行
- [ ] 変数の定義と環境間での値の切り替え

### レポート機能

- [ ] テスト結果のJSON形式出力
- [ ] HTMLレポートの生成
- [ ] JUnitフォーマット対応（CI/CD統合用）
- [ ] 実行時間の測定とパフォーマンス統計

### その他の機能

- [ ] リトライ機能
- [ ] タイムアウト設定
- [ ] プロキシサポート
- [ ] 証明書の検証設定
- [ ] レスポンスの保存機能
- [ ] 環境変数からの設定読み込み

## 開発

```bash
# ビルド
cargo build

# テスト実行
cargo test

# 開発モードで実行
cargo run -- https://httpbin.org/get
```
