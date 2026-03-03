# todo-app

Rust製のシンプルなTodo CLIアプリケーション。

## 機能
- タスクの追加、完了、削除、一覧表示
- タスクの総数表示（`count`）
- 全タスクの一括削除（`clear`）

## ビルド

```bash
cargo build --release
```

## 使い方

```bash
# タスクを追加
todo-app add 買い物に行く

# タスク一覧を表示
todo-app list

# タスクを完了にする
todo-app done 1

# タスクを削除
todo-app remove 1

# タスクの総数を表示
todo-app count

# 全タスクを削除
todo-app clear
```

## コマンド

| コマンド | 説明 |
|---|---|
| `add <title>` | 新しいタスクを追加 |
| `done <id>` | タスクを完了にする |
| `remove <id>` | タスクを削除 |
| `list` | 全タスクを一覧表示 |
| `count` | タスクの総数を表示 |
| `clear` | 全タスクを削除 |
| `count` | タスクの総数を表示 |
| `clear` | 全タスクを削除 |
