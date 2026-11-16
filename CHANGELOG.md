# Changelog

## 0.3.0 (2026-11-15) - Tier 3
- Persistent LLM Settings (`load_llm_config`/`save_llm_config`) in SQLite `settings`.
- Daily JSONL chat transcripts under `%LOCALAPPDATA%\puppycraft\chat\<YYYY-MM-DD>.jsonl` (toggleable).
- Expanded MCP tool surface (`puppycraft_vet_list`, `puppycraft_vet_add`, `puppycraft_parks_list`) + updated `puppycraft-workflow` skill.
- Release binaries built and verified (`puppycraft-app`, `puppycraft-mcp`).

## 0.2.0 (2026-11-15) - Tier 2
- Side-by-Side Breed Comparator (Rasse A vs. B) in Hundeenzyklopädie.
- GPU-rendered Weight Sparkline chart in Hundsi-Tagebuch.
- Sitter Calendar status lifecycle (`Bestätigen`, `Erledigt`, `Löschen`) + emergency handoff export.
- Polaroid frame option + persistent saved photo gallery in Hundsi-Phototool.
- SQLite-backed Dog Parks / Hundezonen and Vet Visits / Tierarzt-Akte, plus Separation Training Plan, Daily Routine Planner, Insurance Checklist, and Senior Care guide.
- Build-time `skills/` completeness test verifying every skill folder is bundled.

## 0.1.0 (2026-11-15)
- Initial Pure-Rust ArtCraft workspace (`puppycraft-core`, `puppycraft-mcp`, `puppycraft-app`).
- 30+ breed dog encyclopedia and family/location matcher (Einsamer Oldie, Familie mit 3 Kids + Katze, Almoehi, Innenstadt, etc.).
- Breeder directory & red-flag checker, rescue/adoption guide, puppy training milestones, trainers & training grounds, DACH/EU legal guide, vaccination tracker, and travel guide (Vienna vs. Tokyo & more).
- Dog diary, partner/relative sitter calendar with handoff export, real image-crate Hundsi photo studio, and 20+ extra dog tools (toxicology/chocolate calculator, feeding calculator, TCO budget, body language, first aid, names, quiz, etc.).
- Bundled `SKILL.md` skills served over `skill://` MCP resources, `puppycraft_skill_read`, CLI `skills`, Help > Skills tab, and `Ctrl+J` AI Chatbot dock.
