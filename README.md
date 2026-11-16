# PuppyCraft

![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)
![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange?logo=rust)
![MCP stdio](https://img.shields.io/badge/MCP-stdio-purple)
![ArtCraft harness](https://img.shields.io/badge/ArtCraft-harness-green)

**Local-first dog acquisition advisor, 30-breed encyclopedia, care and travel hub, and AI chat workstation in pure Rust.** Built for prospective and current dog owners with breed matching, training, DACH/EU legal and travel guidance (e.g. Vienna vs. Tokyo), vaccination tracking, photo studio, dog diary, sitter calendar, and skill-grounded AI chat.

## Features

- **30-Breed Encyclopedia & Matcher**: Interactive breed matching (including presets like `--preset oldie`) and detailed breed profiles.
- **Care, Medical & Legal/Travel Hub**: DACH/EU legal requirements, international travel checklists, and vaccination records in SQLite (`PUPPYCRAFT_DATA`).
- **Diary, Photo Studio & Sitter Calendar**: Daily logs, local image editing, and sitter scheduling.
- **Skill-Grounded AI Chat (`Ctrl+J`)**: Local-first LLM client grounded in bundled `SKILL.md` skills exposed via MCP `skill://` resources.

## The Craft App Standard (3-Tier ArtCraft Architecture)

PuppyCraft implements the **Pure-Rust ArtCraft** 3-tier containment harness:

| Layer | Crate | Role |
|---|---|---|
| **L1 Core** | `crates/puppycraft-core` | Domain models, 30-breed encyclopedia & matcher, DACH/EU legal & travel guide, SQLite store (`PUPPYCRAFT_DATA`), image editor, local-first LLM client, bundled `SKILL.md` skills, and `dispatch_tool` bus with zero-panic lint walls (`unwrap_used` / `expect_used` / `panic` denied). |
| **L2 CLI + MCP** | `crates/puppycraft-mcp` | Single native `puppycraft` binary providing `clap` CLI subcommands and a JSON-RPC `2024-11-05` MCP stdio server (`tools/list`, `tools/call`, `resources/list`, `resources/read` for `skill://` URIs) with zero Python/Node runtime dependencies. |
| **L3 Desktop App** | `crates/puppycraft-app` | Native GPU-rendered `eframe`/`egui` 0.37 workstation with `Ctrl+J` AI Chatbot dock invoking L1 directly in-process. |

**Lineage**: Adapted from the Storytold ArtCraft pattern ([storytold/artcraft](https://github.com/storytold/artcraft)): one workspace, one command bus, headless-first CLI + MCP + native GUI.

## Quick Start

```powershell
# Launch native desktop workstation
cargo run -p puppycraft-app

# CLI & MCP stdio
cargo run -p puppycraft-mcp -- status --json
cargo run -p puppycraft-mcp -- match --preset oldie
cargo run -p puppycraft-mcp -- travel Tokio
cargo run -p puppycraft-mcp -- mcp
```

## MCP Server Configuration

```json
{
  "mcpServers": {
    "puppycraft": {
      "command": "D:/Dev/repos/puppycraft/target/release/puppycraft.exe",
      "args": ["mcp"]
    }
  }
}
```

## Verification

```powershell
just check
```

## License

MIT OR Apache-2.0
