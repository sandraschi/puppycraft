# PuppyCraft

Pure-Rust ArtCraft desktop application, CLI, and MCP server for dog acquisition advice, breed matching, dog encyclopedia, training, legal/travel guidance, vaccinations, photo studio, dog diary, sitter calendar, and skill-grounded AI chat.

## Crates

- `puppycraft-core`: Domain models, 30-breed encyclopedia & matcher, DACH/EU legal & travel guide (Vienna vs. Tokyo), SQLite store (`PUPPYCRAFT_DATA`), image editor, local-first LLM client, bundled `SKILL.md` skills, and `dispatch_tool` bus.
- `puppycraft-mcp`: `clap` CLI and MCP `2024-11-05` stdio server (`tools/list`, `tools/call`, `resources/list`, `resources/read` for `skill://` URIs).
- `puppycraft-app`: `eframe`/`egui` 0.37 native desktop workstation with `Ctrl+J` AI Chatbot dock.

## Quick Start

```powershell
cargo run -p puppycraft-app
```

CLI / MCP:

```powershell
cargo run -p puppycraft-mcp -- status --json
cargo run -p puppycraft-mcp -- match --preset oldie
cargo run -p puppycraft-mcp -- travel Tokio
cargo run -p puppycraft-mcp -- mcp
```

## Verification

```powershell
just check
```
