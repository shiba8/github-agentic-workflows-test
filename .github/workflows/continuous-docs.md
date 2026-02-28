---
on:
  push:
    branches: [main]
    paths:
      - "src/**"
      - "Cargo.toml"

permissions:
  contents: read
  issues: read
  pull-requests: read

safe-outputs:
  create-pull-request:
    title-prefix: "[docs] "
    labels: [documentation]

tools:
  github:
---

# ドキュメント自動更新

mainブランチにソースコードの変更がpushされたら、変更内容を確認し、
README.mdをコードの現状に合わせて更新する。

## 手順

1. 現在のREADME.mdを読む
2. 変更されたソースファイル（src/**）とCargo.tomlを読む
3. コードの変更に基づいてREADMEの更新が必要か判断する
4. 更新が必要な場合、以下を正確に反映するようREADME.mdを修正する:
   - プロジェクトの説明と機能
   - 使い方と利用可能なコマンド
   - ビルドとインストール手順
5. READMEの更新が不要な場合は何もしない
6. READMEは簡潔かつ正確に保つ
