set shell := ["pwsh", "-NoLogo", "-NoProfile", "-Command"]

default:
    @just --list

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace

fmt:
    cargo fmt --all

test:
    cargo test --workspace

build:
    cargo build --workspace --release

run:
    cargo run -p puppycraft-app

mcp:
    cargo run -p puppycraft-mcp -- mcp

smoke:
    cargo run -p puppycraft-mcp -- status --json
    cargo run -p puppycraft-mcp -- skills list --json
