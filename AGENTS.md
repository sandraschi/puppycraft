# PuppyCraft - AGENTS.md

Pure-Rust ArtCraft application for dog acquisition advice, breed matching, encyclopedia, training, legal/travel guidance, vaccinations, photo studio, dog diary, sitter calendar, and skill-grounded AI chat.

## Architecture (Pure-Rust ArtCraft 3-Tier)

- `crates/puppycraft-core` (L1): Domain models, 30+ breed encyclopedia & matcher, knowledge base (breeders, rescue, training, trainers, legal, vaccines, travel, 20+ tools), SQLite store (`PUPPYCRAFT_DATA`), image processing (`photo.rs`), local-first LLM client (`llm.rs`), bundled skills (`skills.rs`), and `Command::dispatch` bus.
- `crates/puppycraft-mcp` (L2): `clap` CLI + MCP `2024-11-05` stdio server (`tools/list`, `tools/call`, `resources/list`, `resources/read` for `skill://` URIs).
- `crates/puppycraft-app` (L3): Pure-Rust `eframe`/`egui` 0.37 native desktop workstation with `Ctrl+J` AI Chatbot dock.

## Gates

Run `just check`:
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace`
