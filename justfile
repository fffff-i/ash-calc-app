# Windows PowerShell をシェルとして使用
set shell := ["powershell", "-NoProfile", "-Command"]

# Rust のWASMビルド + 出力ファイルをweb/srcにコピー
build-core:
    wasm-pack build core --target web --out-dir pkg --out-name ash_calc_app
    if (!(Test-Path "web/src/pkg")) { New-Item -ItemType Directory -Path "web/src/pkg" }
    Get-ChildItem "core/pkg" | Copy-Item -Destination "web/src/pkg" -Force

# localで動かす
start-dev:
    just build-core
    cd web; npm run dev

# Rust のテスト実行
test-core:
    cargo test --manifest-path core/Cargo.toml
