---
marp: true
theme: default
paginate: true
header: "GitHub Agentic Workflows"
footer: "Lightning Talk - 2026/03"
style: |
  section {
    font-family: 'Helvetica Neue', Arial, 'Hiragino Kaku Gothic ProN', sans-serif;
  }
  h1 {
    color: #24292f;
  }
  h2 {
    color: #0969da;
  }
  code {
    background: #f6f8fa;
  }
  table {
    font-size: 0.85em;
  }
  section.title {
    text-align: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }
  section.title h1 {
    font-size: 2.5em;
    color: #0969da;
  }
---

<!-- _class: title -->

# GitHub Agentic Workflows

AIエージェントでリポジトリタスクを自動化する

---

## このLTで話すこと

1. **GitHub Agentic Workflows とは？**
2. **仕組みとアーキテクチャ**
3. **セキュリティ設計**
4. **実際にセットアップしてみた**
5. **ハマったポイントと学び**

---

## GitHub Agentic Workflows とは

- **Markdownで書いた指示をAIエージェントがGitHub Actions上で実行**する仕組み
- 2026年2月〜 Technical Preview
- 従来のCI/CDを**補完**するもの（置き換えではない）

```
Markdownで意図を記述 → コンパイル → GitHub Actionsで実行 → AI が処理
```

---

## 何が嬉しいのか

従来のGitHub Actions:
```yaml
# 全てのロジックを自分でYAMLで書く
- run: |
    grep -r "TODO" src/ > report.txt
    # 100行のシェルスクリプト...
```

Agentic Workflows:
```markdown
ソースコードの変更を確認し、
README.mdをコードの現状に合わせて更新する。
```

**自然言語で「意図」を書くだけでAIが判断・実行してくれる**

---

## ユースケース

| パターン | 内容 |
|---|---|
| **Issue自動トリアージ** | Issue の要約・ラベリング・ルーティング |
| **ドキュメント自動更新** | コード変更に合わせてREADME更新PRを作成 |
| **コード品質改善** | 改善点を見つけてPRを自動作成 |
| **テスト改善** | カバレッジ評価と高価値テストの追加 |
| **CI失敗の調査** | CI失敗の原因調査と修正提案 |
| **日次レポート** | リポジトリの定期ヘルスレポート生成 |

---

## ファイル構成

`.github/workflows/` に2つのファイルを置く

| ファイル | 役割 | 誰が書く |
|---|---|---|
| `continuous-docs.md` | ワークフロー定義 | **人間** |
| `continuous-docs.lock.yml` | 実行用Actions定義 | **`gh aw compile`** |

lock.yml は `package-lock.json` と同じ立ち位置
**手で編集しない、コンパイルで再生成する**

---

## `.md` ファイルの全体構造

```yaml
---
on:            # いつ実行するか（トリガー）
permissions:   # GITHUB_TOKENの権限スコープ
safe-outputs:  # エージェントに許可する書き込み操作
engine:        # 使用するAIエージェントとモデル
tools:         # エージェントが使えるツール
---
# ここからMarkdownでAIへの指示を書く（プロンプト）
```

フロントマター（YAML）= **設定**、Markdown本文 = **AIへの指示**

---

## `permissions:` — GITHUB_TOKENの権限スコープ

GitHub Actions が自動発行する `GITHUB_TOKEN` に**どの操作を許可するか**を定義する

```yaml
permissions:
  contents: read        # リポジトリのコード・ファイル
  issues: read          # Issue
  pull-requests: read   # Pull Request
```

各項目に `read` / `write` / なし（権限なし）を設定可能

| 対象 | 説明 |
|---|---|
| `contents` | リポジトリのコード・ファイル |
| `issues` / `pull-requests` | Issue / PR の閲覧・操作 |
| `actions` / `packages` / `security-events` | その他 |

---

## `safe-outputs:` — エージェントに許可する書き込み操作

エージェント自体は読み取り専用。**書き込み操作はここでホワイトリスト定義する**

```yaml
safe-outputs:
  create-pull-request:           # PR作成を許可
    title-prefix: "[docs] "     # タイトルにプレフィックス強制
    labels: [documentation]     # ラベル自動付与
```

指定できる操作の種類:

| safe-output | 内容 |
|---|---|
| `create-pull-request` | PR を作成する |
| `create-issue` | Issue を作成する |
| `add-comment` | Issue/PR にコメントする |
| `add-reviewer` | PR にレビュアーを追加する（最大3人） |
| `assign-to-user` | Issue にユーザーをアサインする |
| `dispatch-workflow` | 他のワークフローをトリガーする |

---

## フロントマター = 設定、Markdown = プロンプト

```
┌─────────────────────────┐
│  フロントマター（YAML）  │ → パースして設定値を抽出
│  on, permissions, engine │     ↓
│  safe-outputs, tools     │   固定テンプレートに埋め込み
├─────────────────────────┤     ↓
│  Markdown本文            │ → そのままAIへのプロンプト
│  「READMEを更新して」    │   （変換しない）
└─────────────────────────┘
         ↓
   lock.yml（固定テンプレート + 設定値）
```

`gh aw compile` は**AIではなくテンプレートエンジン**
フロントマターが同じなら出力も同じ（冪等）

---

## セキュリティ設計: Defense-in-Depth

AIに直接書き込み権限を渡すのは危険 → **多層防御**

```
agent ジョブ（読み取り専用）
  → コードを読んで変更内容を生成
  → 「こう変更したい」と safe-output に出力
      ↓
safe_outputs ジョブ（書き込み権限あり）
  → エージェントの出力を検証
  → 安全と判断したらPRを実際に作成
```

---

## セキュリティまとめ

| 対策 | 内容 |
|---|---|
| **デフォルト読み取り専用** | エージェントは書き込み不可 |
| **safe-outputs** | 事前承認された操作のみ実行可能 |
| **サンドボックス** | 隔離された環境で実行 |
| **ネットワーク隔離** | 外部アクセスを制限 |
| **PRの自動マージ不可** | 人間のレビュー・承認が必須 |
| **SHAピン止め依存関係** | サプライチェーン攻撃を防止 |

---

## 対応AIエージェント

| エージェント | 必要なシークレット |
|---|---|
| **GitHub Copilot** | `COPILOT_GITHUB_TOKEN` (PAT) |
| **Claude Code** | `ANTHROPIC_API_KEY` |
| **OpenAI Codex** | `OPENAI_API_KEY` |

- Copilot Free（無料）でも利用可能
- Premium Requests を消費（Free: 月50回、1実行で約2回消費）
- モデルは `engine.model` で指定可能（GPT-4.1, Claude Sonnet 4.6 など）

---

## 実際にやってみた: セットアップ手順

```bash
# 1. CLI拡張をインストール
gh extension install github/gh-aw

# 2. .github/workflows/ に .md ファイルを作成
#    （フロントマター + Markdownの指示）

# 3. コンパイル
gh aw compile

# 4. .gitattributes を追加
echo '.github/workflows/*.lock.yml linguist-generated=true merge=ours' \
  > .gitattributes

# 5. コミット & プッシュ
```

---

## 実際にやってみた: リポジトリ設定

### トークン設定
- Fine-grained PAT を作成（リポジトリ単位で権限を絞れる）
- Permission: **Copilot Requests → Read**
- Repository secrets に `COPILOT_GITHUB_TOKEN` として登録

### Actions権限
- Settings → Actions → General
- **「Allow GitHub Actions to create and approve pull requests」** を有効化

---

## ハマりポイント 1: モデル未有効化

```
Error: Run `copilot --model claude-sonnet-4.6` in interactive mode
to enable this model
```

- デフォルトモデルが `claude-sonnet-4` だが、アカウントで未有効化だった
- **対策**: `engine.model` を明示指定して回避

```yaml
engine:
  id: copilot
  model: gpt-4.1    # 有効化済みのモデルを指定
```

---

## ハマりポイント 2: エージェントがPRを作らない

```json
{"items":[{"message":"README.md updated...",
  "type":"noop"}]}
```

- エージェントは「更新が必要」と判断したのに noop で終了
- safe-outputs の `create-pull-request` を**呼ばなかった**
- **対策**: プロンプトに明示的な指示を追加

```markdown
# Before
4. 更新が必要な場合、README.mdを修正する

# After
4. 更新が必要な場合、README.mdを修正する
5. 必ず create-pull-request を使ってPRを作成する  ← 追加
```

---

## ハマりポイント 3: PR作成の権限エラー

```
GitHub Actions is not permitted to create or approve pull requests.
```

- lock.yml にジョブレベルで `pull-requests: write` があっても
  **リポジトリ設定のスイッチがOFF** だとブロックされる
- Settings → Actions → General で有効化が必要

---

## 学びまとめ

1. **プロンプトは明示的に** — ツールを定義しても「いつ使うか」まで指示しないとAIは使わないことがある

2. **セキュリティは多層構造** — エージェント（読み取り専用）と safe-outputs（書き込み）の分離が重要

3. **設定は3箇所** — `.md`ファイル、Repository Secrets、Actions権限設定の全てが必要

4. **`gh aw compile` はAI処理ではない** — テンプレート変換なので冪等。フロントマターが同じなら出力も同じ

---

## 料金感覚

| プラン | Premium Requests/月 | 月あたり実行可能回数 |
|---|---|---|
| Copilot Free | 50 | 約25回 |
| Copilot Pro ($10) | 300 | 約150回 |
| Copilot Pro+ ($39) | 1,500 | 約750回 |

- 1実行 ≒ 2 Premium Requests
- GitHub Actions minutes は公開リポジトリなら無制限
- リセットは毎月1日 UTC 00:00（日本時間 9:00）

---

## まずは試してみよう

最小構成で始められる:
- **公開リポジトリ** + **Copilot Free** + **Fine-grained PAT**
- Issue自動ラベリングやREADME自動更新から始めるのがおすすめ

### 参考リンク
- [公式ブログ](https://github.blog/ai-and-ml/automate-repository-tasks-with-github-agentic-workflows/)
- [gh-aw リポジトリ](https://github.com/github/gh-aw)
- [ドキュメント](https://github.github.com/gh-aw/)

---

<!-- _class: title -->

# Thank you!

質問があればお気軽にどうぞ
