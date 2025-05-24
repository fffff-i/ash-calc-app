# bashシェルを使用する
set shell := ["bash", "-cu"]

# Rust のWASMビルドをweb/srcに出力
build-core:
    wasm-pack build core --target web --out-dir ../web/src/pkg --out-name ash_calc_app

# localで動かす
start-dev:
    just build-core
    cd web && npm run dev

# Rust のテスト実行
test-core:
    cargo test --manifest-path core/Cargo.toml
