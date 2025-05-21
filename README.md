# Ash Calc App

**アッシュテイル ～風の大陸～** 向けのダメージ計算・攻撃指数可視化ツール。
Rust + WebAssembly による計算ロジック（`core/`）と、今後追加予定のフロントエンド（`web/`）で構成されたモノレポです。

---

## 📁 ディレクトリ構成

```
ash-calc-app/
├── core/          # Rust製計算エンジン（wasm-pack対象）
│   ├── src/
│   ├── pkg/       # wasm出力（.gitignore済）
│   └── Cargo.toml
├── web/           # （予定）TypeScript + Vite フロントエンド
├── justfile       # 共通ビルド・起動スクリプト（Rust用）
└── README.md
```

---

## 🚀 開発用コマンド（要：just, wasm-pack）

### 初期セットアップ

```bash
cargo install just
npm install -g wasm-pack
```

### Rust のビルド（WASM出力）

```bash
just build-core
```

### Rust のテスト実行

```bash
just test-core
```

※ フロントエンドの構成は今後検討予定です。

---

## 🧩 公開API（JSから呼び出し）

| 関数名                                          | 説明                   |
| -------------------------------------------- | -------------------- |
| `calc_damage_with_skills_js(params, skills)` | 通常の最終ダメージ計算          |
| `calc_damage_verbose_js(params, skills)`     | 補足情報付き（非会心・会心・期待値など） |
| `get_skill_list()`                           | スキル選択UI用のマスターデータ取得   |

---

## 📦 技術構成

* Rust 2024 edition + wasm-bindgen
* wasm-pack
* just（コマンドランナー）
* モノレポ構成（`core/`, `web/`）

---

## 📝 ライセンス

MIT License
