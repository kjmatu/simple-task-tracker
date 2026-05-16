# task-tracker (ttk) 仕様まとめ

## プロジェクト概要

- **目的**: 会社の稼働時間を記録し、タスク別に時間を集計する個人用ツール
- **プラットフォーム**: macOS
- **リポジトリ名**: `task-tracker`(仮、`ttk`でも可)
- **コマンド名**: `ttk`
- **設定ディレクトリ**: `~/.config/ttk/`

## 開発フェーズ

### フェーズ1: core + CLI(MVP、ここから着手)

- SQLiteではなくJSONベースで開始
- start / stop / status / log / export
- ラベル管理(labels.tomlによるユーザー定義)
- 業務日ロジック(翌9時境界)

### フェーズ1.5: 運用知見の反映

- stop忘れ復旧、last編集、レポート機能、シェル補完など

### フェーズ2: Tauriでメニューバー常駐GUI

- トグルUI、当日合計、ラベル選択
- ポップオーバー型(トレイアイコン直下に小窓)
- `activation_policy = Accessory`でDockから消す
- iconAsTemplate: trueでダークモード対応

### フェーズ3: ポモドーロ + デーモン化

- `ttkd`を作り、launchdで常駐
- CLI/GUIはデーモンクライアントに
- Unix domain socket経由でIPC
- 通知、自動休憩、サイクル管理

## アーキテクチャ

### Cargo workspace構成

```
task-tracker/
├── Cargo.toml              # workspace定義
└── crates/
    ├── core/               # ロジック本体(ライブラリ)
    ├── cli/                # CLIバイナリ(ttk)
    └── tauri-app/          # Tauriアプリ(フェーズ2以降)
```

### 設計原則

- **coreは副作用の入口を引数で受ける**: グローバル状態を持たない
- **Storage trait で永続化を抽象化**: JSON/SQLite切り替え可能にする
- CLIとGUIはcoreを呼ぶ薄いラッパー

```rust
pub trait Storage {
    fn start(&mut self, label_id: &str, note: Option<&str>) -> Result<Session>;
    fn stop(&mut self) -> Result<Session>;
    fn status(&self) -> Result<Status>;
    fn query(&self, range: DateRange) -> Result<Vec<Session>>;
}
```

## データモデル

### Session

```
- id            (UUID)
- start_at      (ISO 8601)
- end_at        (ISO 8601, nullなら稼働中)
- label_id      (nullable, labels.tomlのidを参照)
- note          (任意メモ)
- kind          ("work"固定、将来の拡張用: short_break/long_break)
```

### Label(labels.tomlで管理)

```toml
[[labels]]
id = "dev"            # 不変、セッションから参照される
name = "開発"          # 表示名、自由に変更可
color = "#3b82f6"
archived = false      # 論理削除

[[labels]]
id = "meeting"
name = "会議"
color = "#ef4444"
```

## ストレージ(フェーズ1)

### 方針: JSON、月別分割

```
~/.config/ttk/
├── config.toml
├── labels.toml
├── state.json              # 現在稼働中のセッション
└── sessions/
    └── YYYY-MM.json        # 月単位で分割
```

### 選定理由

- 1日10セッション、5年でも約18,000件、数MB程度 → JSONで十分
- 目視確認・手編集・Gitバックアップ可能
- SQLiteへの移行はStorage trait経由でいつでも可能

### アトミック書き込み必須

```rust
// tempfile → rename パターン、または atomicwrites クレート
```

### SQLiteに移行すべき兆候

- 集計クエリが遅い(数百ms超)
- 複数プロセス同時書き込み構成になる
- レコード数が数万超
- 複雑な検索が頻繁

## 業務日ロジック

**ルール**: 翌朝9:00を日付境界とする(深夜作業は前日扱い)

```
business_date(ts):
    if ts.hour < 9:
        return (ts - 1日).date()
    else:
        return ts.date()
```

- セッションは`start_at`基準で業務日を判定(分割しない方針)
- 9:00境界は`config.toml`で変更可能にしておく
- `stop`忘れ対策: `start`時に前回未クローズかつ翌9:00超過なら自動クローズまたは確認

## ラベル管理仕様

### 方針: ファイル(labels.toml)が真実

- プリセットなし、ユーザーが自由にCRUD
- UIで編集したい場合は「設定ファイルを開く」でエディタ起動
- `id`は不変、`name`は変更自由(過去セッション破壊されない)

### 起動時バリデーション

- id重複、空文字、未知フィールドをチェック
- エラーは行番号付きで親切に表示

### 削除は論理削除(`archived = true`)

- sessionsから参照されてるラベルは残す
- UIの選択肢からは消える、集計には残る
- name uniqueは`archived = false`のものだけで制約

### 欠損ラベル対応

- sessionのlabel_idがlabels.tomlから消えた場合、`(deleted: dev)`表示で履歴は保持

## CLIコマンド

### 基本操作

```
ttk start <label> [--note "..."]
ttk stop
ttk status
ttk switch <label>          # 今のを止めて別ラベルで即start
ttk log [--today/--week/--month]
ttk export [--from] [--to] [--out file.csv]
ttk report --by label --week
```

### ラベル管理

```
ttk labels list [--all]
ttk labels check            # labels.toml検証
```

※編集は基本labels.tomlを直接。補助コマンドも用意可能。

## CSV出力形式

```
id, business_date, label, start_at, end_at, duration_minutes, note
```

- `business_date`列を別途持つ(Excelピボットで一発集計可能)
- labelはid参照でなく**名前を埋め込む**(後で読めるように)

## Tauri メニューバーUI(フェーズ2)

### 構成

- ポップオーバー型(ネイティブメニューではなくWebViewの小窓)
- トグル1つ(OFF→start、ON→stop)
- ラベル選択プルダウン(最近使った順)
- 当日合計をラベル別積み上げ表示
- 稼働中は経過時間とアイコン色で識別

### 技術メモ

- Tauri v2の`TrayIcon` API使用
- `activation_policy = Accessory`でメニューバー専用アプリ化
- 参考実装: `ahkohd/tauri-macos-menubar-app-example`
- `#[tauri::command]`でstart/stop/statusをフロントに公開
- `Mutex<Punch>`をstate管理

## ポモドーロ(フェーズ3)

### 設計方針

- 基本は独立レイヤー(稼働記録の上に乗せる)
- 休憩時間も記録したい場合は`sessions.kind`を活用

### config.toml設定例

```toml
[pomodoro]
work_minutes = 25
short_break_minutes = 5
long_break_minutes = 15
cycles_before_long_break = 4
auto_start_break = false
auto_start_work = false
notification_sound = true
```

### デーモン化(`ttkd`)

- Unix domain socket: `~/Library/Application Support/ttk/ttkd.sock`
- launchdで常駐(`~/Library/LaunchAgents/com.yourname.ttkd.plist`)
- CLIからはデーモンへJSON-RPC送信
- 通知: `notify-rust`クレートまたはTauri plugin

### CLIコマンド追加

```
ttk start <label> --pomodoro [minutes]
ttk pomodoro status
ttk pomodoro skip
ttk pomodoro stats --today
ttk daemon install/uninstall/status/restart/logs
```

## 今やっておくこと(将来の拡張余地)

1. `sessions.kind`列を最初から用意(今は`"work"`固定)
1. core logicを状態機械として書く(idle ⇔ running、将来break追加)
1. `config.toml`に`[pomodoro]`セクション予約

## 主要crate候補

- `serde`, `serde_json`, `toml` - シリアライズ
- `clap` - CLI引数パース
- `chrono` or `time` - 日時
- `uuid` - ID生成
- `thiserror` - エラー型
- `dirs` - 設定ディレクトリ解決
- `atomicwrites` - アトミック書き込み
- `notify-rust` - 通知(フェーズ3)
- `tokio` - 非同期(フェーズ3デーモン)
- `ratatui` - TUI(採用するなら)
- Tauri v2(フェーズ2)

## 決定事項まとめ

- 言語: Rust
- 永続化: JSON(月別分割)→ 必要になればSQLite
- 業務日: 翌9時境界(設定可変)
- ラベル: labels.tomlでユーザー管理、論理削除
- コマンド名: `ttk`
- 配布形態: CLI単体 → GUI追加 → デーモン化の順

## 未決事項

- ラベル未指定start時の挙動(エラー/未分類/前回踏襲)
- 過去セッションのラベル後付け変更の可否
- 複数ラベル併用の可否(当面は単一)
- TUIを挟むかどうか(現状はスキップ案)