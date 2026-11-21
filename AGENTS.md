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


## Documentation preservation and hooks

On 2026-11-20 a README "standardization" sweep across the fleet deleted project descriptions,
status sections and user warnings in nine repos. Full rules:
`mcp-central-docs/standards/CONTENT_PRESERVATION_STANDARD.md`.

- Applying any standard or template restructures; it never deletes. Move content (within the file
  or to `docs/` with a link), do not drop it. Never remove what the project is, what it is based on,
  its status (what works, what does not, what is not built) or warnings to users.
- Remove content only when Sandra explicitly asks for that removal, in this repo.
- Never add claims you have not verified (release downloads, versions, test counts, features).
- Only standards with `status: active` may be applied across repos. A change across several repos is
  done on one repo first and approved by Sandra; its commits name the tool and carry a `Sweep-Id:` line.
- `.githooks/commit-msg` blocks a commit that deletes a Markdown file, removes 20% of its lines
  (15 or more) or removes 3 or more headings. A removal Sandra asked for passes with
  `Docs-Removal-Approved: <who and why>` in the message. Enable once per clone:
  `git config core.hooksPath .githooks`. Never use `--no-verify`; if a hook blocks you, stop and ask.
