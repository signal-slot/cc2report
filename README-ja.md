# cc2report

*他の言語で読む: [English](README.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Claude Code の会話ログを分析し、AI を使用して包括的な作業報告書を生成するインテリジェントな作業報告書ジェネレーターです。

## 機能

- **AI 駆動の分析**: OpenAI の GPT モデルを使用して会話を分析し、インテリジェントな要約を生成
- **多言語サポート**: 19言語でレポートを生成（システムロケールから自動検出）
- **スマートキャッシング**: API レスポンスをキャッシュしてコストを削減し、パフォーマンスを向上（キャッシュ時は12倍高速）
- **並列処理**: 複数のプロジェクトを同時に処理して分析を高速化
- **柔軟な日付フィルタリング**: 特定の日付、週、月のレポートを生成
- **進捗インジケーター**: 長時間実行される操作のための視覚的な進捗バー
- **トークン使用状況の追跡**: API の使用状況とコストを監視
- **テンプレートのカスタマイズ**: AI プロンプトとレポート形式をカスタマイズ

## インストール

### ソースから

```bash
# リポジトリをクローン
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# ビルドしてインストール
cargo build --release
cargo install --path .
```

### 必要条件

- Rust 1.70 以上
- OpenAI API キー

## 設定

### API キーのセットアップ

OpenAI API キーを環境変数として設定：

```bash
export OPENAI_API_KEY="your-api-key-here"
```

またはコマンドラインで直接指定：

```bash
cc2report --api-key "your-api-key-here"
```

### デフォルトパス

- **ログディレクトリ**: `~/.claude/projects/`（Claude Code のデフォルトプロジェクトディレクトリ）
- **キャッシュディレクトリ**: `~/.cache/cc2report/`
- **テンプレートファイル**: `./cc2report.toml` または `~/.config/cc2report/templates.toml`

## 使用方法

### 基本的な使用方法

今日の会話のレポートを生成（デフォルト）：

```bash
cc2report
```

すべての会話のレポートを生成（日付フィルタなし）：

```bash
cc2report --all
```

### 日付フィルタリング

```bash
# 特定の日付
cc2report --date 2024-07-01

# 日付範囲
cc2report --from 2024-07-01 --to 2024-07-07

# 今週
cc2report --weekly

# 今月
cc2report --monthly
```

### 出力オプション

```bash
# ファイルに保存
cc2report --output report.md

# JSON 形式
cc2report --format json --output report.json

# 言語を指定（デフォルトは自動検出）
cc2report --lang ja  # 日本語
cc2report --lang zh  # 中国語
cc2report --lang es  # スペイン語

# 例：今日のレポートを日本語で
cc2report --lang ja

# 例：今週のレポートをスペイン語で
cc2report --weekly --lang es
```

### パフォーマンスオプション

```bash
# 並列処理を有効化（最大10）
cc2report --parallel 4

# 進捗インジケーターを無効化
cc2report --quiet

# トークン使用状況とコストを表示
cc2report --show-token-usage
```

### キャッシュ管理

```bash
# キャッシュをクリア
cc2report --clear-cache

# キャッシュ情報を表示
cc2report --cache-info
```

### テンプレートのカスタマイズ

テンプレートファイルを生成：

```bash
cc2report --generate-template my-template.toml
```

テンプレートを編集してプロンプトとレポート形式をカスタマイズ。

## サポートされている言語

このツールはシステム言語を自動的に検出し、それに応じてレポートを生成します。サポートされている言語：

- 英語 (en)
- 日本語 (ja)
- 中国語 (zh)
- 韓国語 (ko)
- スペイン語 (es)
- フランス語 (fr)
- ドイツ語 (de)
- ポルトガル語 (pt)
- ロシア語 (ru)
- イタリア語 (it)
- オランダ語 (nl)
- ポーランド語 (pl)
- トルコ語 (tr)
- アラビア語 (ar)
- ヒンディー語 (hi)
- タイ語 (th)
- ベトナム語 (vi)
- インドネシア語 (id)
- マレー語 (ms)

## レポート構造

生成されるレポートには以下が含まれます：

- **プロジェクトタイトルと目的**: 作業内容の明確な要約
- **アクティビティ**: 実行された作業のカテゴリ別リスト
- **提供された価値**: 具体的な成果と実装された機能
- **技術的改善**: コード品質とパフォーマンスの向上
- **未解決の問題**: ブロッカーまたは保留中のタスク

## コスト最適化

このツールには API コストを最小限に抑えるためのいくつかの機能が含まれています：

1. **キャッシング**: レスポンスは24時間キャッシュされます
2. **モデル選択**: コスト/品質のトレードオフに基づいて異なるモデルを選択
3. **トークン追跡**: 予算内に収まるように使用状況を監視

### モデル価格（2024年現在）

| モデル | 入力コスト | 出力コスト | 推奨 |
|-------|------------|-------------|------|
| gpt-4o（デフォルト） | $2.50/1M | $10.00/1M | 最高品質 |
| gpt-4o-mini | $0.15/1M | $0.60/1M | ベストバリュー |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | 予算重視 |

## レポート例

```markdown
# 作業報告書 - 2024-07-01

## cc2report - 作業報告書ジェネレーター

**目的**: Claude Code のログから人間が読める作業報告書を生成

**アクティビティ**:
- 開発 - GPT-4 との OpenAI API 統合を実装
- 機能追加 - インテリジェントな分類によるスマート分析を開発
- UI 強化 - コマンドラインインターフェースの体験を改善

**提供された価値**:
- 「その他のタスク」カテゴリを90%削減
- レポートの可読性を大幅に改善
- 多言語サポート（19言語）を実現

**技術的改善**:
- パフォーマンスを最適化
- エラーハンドリングを強化
```

## トラブルシューティング

### よくある問題

1. **"OpenAI API key is required"**
   - 環境に `OPENAI_API_KEY` が設定されていることを確認
   - または `--api-key` オプションを使用

2. **"Log directory does not exist"**
   - `--log-dir` で正しいパスを指定
   - デフォルトは `~/.claude/projects/`

3. **レート制限エラー**
   - 並列リクエストを削減: `--parallel 1`
   - 下位のモデルを使用: `--model gpt-3.5-turbo`

## 開発

### ソースからのビルド

```bash
# 開発ビルド
cargo build

# テストを実行
cargo test

# デバッグ出力で実行
RUST_LOG=debug cargo run
```

### アーキテクチャ

プロジェクトは以下のモジュールで構成されています：

- `parser`: JSONL ログファイルの解析
- `conversation_analyzer`: 会話からトピックとコンテキストを抽出
- `ai_analyzer`: OpenAI API 統合
- `smart_analyzer`: レポート生成
- `cache`: API レスポンスキャッシング
- `templates`: カスタマイズ可能なプロンプトと形式
- `config`: 設定管理
- `error`: エラーハンドリング
- `cli`: コマンドラインインターフェース

## 貢献

貢献を歓迎します！ガイドラインについては [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

## ライセンス

このプロジェクトは MIT ライセンスの下でライセンスされています - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 謝辞

- [Claude Code](https://github.com/cline/cline)（旧 Claude Engineer）のために構築
- インテリジェントな分析のために OpenAI の GPT モデルを使用
- AI 支援開発における自動化された作業報告の必要性から着想を得ました