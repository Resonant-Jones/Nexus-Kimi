# List commands
default:
    @just --list

# Install deps once
bootstrap:
    npm --prefix apps/desktop install

# Hot-reload desktop
dev:
    npm --prefix apps/desktop run tauri dev

# Lint & test everything
check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace