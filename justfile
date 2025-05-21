# Rust WASM をビルド（出力: core/pkg）
build-core:
    cd core && wasm-pack build --target web --out-dir pkg

# TypeScript フロントエンドを起動（Vite dev server）
start-web:
    cd web && npm run dev

# wasm をビルドしてから Vite を起動（開発開始用）
start:
    just build-core
    just start-web

# フロントの本番ビルド（Vite）
build-web:
    cd web && npm run build

# wasm と web をまとめて本番ビルド
build:
    just build-core
    just build-web

# Rust テスト（coreディレクトリのみに作用）
test-core:
    cargo test --manifest-path core/Cargo.toml
