# simple-task-tracker

タスク別の作業時間を記録するための、Rust製CLIツールです。現在は開発初期段階で、TOMLファイルからのタスク一覧表示に対応しています。

`start`・`stop`・`status`はメッセージを表示する仮実装です。時間の計測や状態の保存はまだ行いません。

## 必要環境

- Rust / Cargo（Edition 2024に対応したRust 1.85以降）

## クイックスタート

リポジトリのルートで実行します。

```sh
cargo build
cargo run -- --help

# サンプルファイルからタスクを表示
STT_TASK_FILE=fixtures/tasks.toml cargo run -- list
```

一覧表示の出力例:

```text
Reading tasks from fixtures/tasks.toml
1: Code Review
2: Develop Feature 1
3: Write Documentation
```

バイナリをインストールする場合:

```sh
cargo install --path .
simple-task-tracker --help
```

## タスクファイル

既定では `$HOME/.config/simple-task-tracker/tasks.toml` を読み込みます。初回はディレクトリとファイルを用意してください。

```sh
mkdir -p "$HOME/.config/simple-task-tracker"
cp fixtures/tasks.toml "$HOME/.config/simple-task-tracker/tasks.toml"
cargo run -- list
```

ファイルは次の形式で記述します。`id`と`name`はどちらも文字列です。

```toml
[[tasks]]
id = "1"
name = "Code Review"

[[tasks]]
id = "2"
name = "Develop Feature 1"
```

環境変数 `STT_TASK_FILE` を設定すると、既定のパスより優先されます。相対パスは実行時のカレントディレクトリを基準に解決されます。

```sh
STT_TASK_FILE=/path/to/tasks.toml cargo run -- list
```

ファイルが存在しない場合、読み込めない場合、またはTOMLの形式が不正な場合、`list`はエラーを表示し終了コード`1`で終了します。

## コマンド

以下はインストール後の実行例です。ソースから実行する場合は、`simple-task-tracker`を`cargo run --`に置き換えてください。

| コマンド | 現在の動作 |
| --- | --- |
| `simple-task-tracker list` | タスクファイルを読み込み、`id: name`形式で一覧表示 |
| `simple-task-tracker start <TASK_ID>` | `Starting <TASK_ID>`を表示（仮実装） |
| `simple-task-tracker stop` | `Stopping`を表示（仮実装） |
| `simple-task-tracker status` | `Status running`を表示（仮実装） |
| `simple-task-tracker --help` | ヘルプを表示 |
| `simple-task-tracker --version` | バージョンを表示 |

`start`の`TASK_ID`は現在`0`〜`255`の整数のみ受け付けます。タスクファイルのIDとの照合は行いません。`status`の表示も実際の稼働状態を表すものではありません。

## プロジェクト構成

```text
src/main.rs         CLI引数とサブコマンドの定義
src/subcommands.rs  コマンド処理とタスクファイルの読み込み
fixtures/tasks.toml 一覧表示用のサンプルデータ
Spec.md             将来の機能・設計案
```

[Spec.md](Spec.md)には、作業時間の保存・集計、CSV出力、GUIなどの構想をまとめています。仕様案に記載された`ttk`というコマンド名や保存先、機能には未実装のものが含まれます。現在の使い方はこのREADMEを参照してください。

## ライセンス

[MIT License](LICENSE)
