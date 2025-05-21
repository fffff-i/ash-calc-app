# Ash Calc App ![CI](https://github.com/fffff-i/ash-calc-app/actions/workflows/ci.yml/badge.svg)

アッシュテイル向けの攻撃指数計算Webアプリケーション。
Rust + WebAssembly + TypeScript構成で、攻撃/防御ステータス・状態異常・スキル効果などを考慮した期待値を出力します。

## 🛠 技術スタック

| レイヤ     | 技術                              |
|---------|---------------------------------|
| フロントエンド | Vite + TypeScript + TailwindCSS |
| バックエンド  | Rust + wasm-pack                |
| 開発支援    | just (タスクランナー)                  |

---

## 📁 ディレクトリ構成

```
ash-calc-app/
├── .github/
│   └── workflows/         # GitHub Actions用CI設定 (ci.yml)
├── core/                  # Rust (WASMビルド対象)
│   ├── src/
│   │   ├── types/         # パラメータ/値の型定義
│   │   │   ├── mod.rs
│   │   │   ├── params.rs
│   │   │   └── value.rs
│   │   ├── calc.rs        # ダメージ計算ロジック
│   │   ├── lib.rs         # wasm_bindgenエントリポイント
│   │   └── skill.rs       # スキルデータと補正
│   ├── Cargo.toml
├── web/                   # フロントエンド (Vite)
│   ├── src/
│   │   ├── pkg/           # wasm-pack出力（Copy済）
│   │   ├── main.ts
│   │   ├── style.css
│   │   └── vite-env.d.ts
│   ├── index.html
│   ├── package.json
│   ├── package-lock.json
│   ├── tsconfig.json
│   └── vite.config.ts
├── justfile               # ビルド/サーバ起動タスク
└── README.md
```

---

## 🚀 開発手順

### 事前準備

- Rust toolchain
- Node.js

### 初回セットアップ

```sh
cargo install just
cargo install wasm-pack
cd web && npm install
```

### 開発ビルド・起動

```sh
just start-dev
```

内部では以下が実行されます：

1. `core/` 内の Rust ソースを WebAssembly にビルド
2. 生成された `pkg` を `web/src/pkg` にコピー
3. Vite 開発サーバーを起動（[http://localhost:5173）](http://localhost:5173）)

---

## 💡 使用方法

画面上で以下のパラメータを入力：

* 攻撃力・攻撃倍率
* 防御力・防御倍率・防御無視
* 会心率・会心ダメージ
* スキル効果（追加補正）
* 敵状態（HP割合、状態異常、ボス/魔物）

🧮 「計算」ボタンを押すと、非会心時/会心時の攻撃指数と期待値を出力します。
各項目に対する **ダメージ上昇率の寄与** も可視化予定です。

---

## 📝 今後の展望

* スキル選択UIの実装
* ステータス保存/読み込み
* ダメージ上昇率グラフ化
* モバイルUI最適化

---

## ⚖️ ライセンス

このリポジトリには公式ゲームの素材やデータは含まれておらず、純粋な計算ツールです。ゲーム運営・開発元とは一切関係ありません。
