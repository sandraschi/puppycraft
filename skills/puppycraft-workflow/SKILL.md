---
name: puppycraft-workflow
description: How to use PuppyCraft tools for breed matching, diary, sitter calendar, vaccines, and status.
---

# PuppyCraft Workflow Skill

Use PuppyCraft's local SQLite-backed tools to inspect and manage the user's dog profile, diary, sitter calendar, and vaccinations.

## Read-Only Tools (run directly)

- `puppycraft_status`: Overview of active dog profiles, diary count, upcoming sitter handoffs, and due vaccines.
- `puppycraft_breed_info`: Detailed encyclopedia profile for a breed by name or search query.
- `puppycraft_match_breeds`: Score and rank breeds for a household/location profile (presets: `oldie`, `family_kids_cat`, `almoehi`, `city_single`, `sporty`).
- `puppycraft_travel_guide`: Dog-friendliness, transit, and entry rules for cities/countries (e.g., `Wien`, `Tokio`, `Deutschland`, `Italien`, `Kroatien`).
- `puppycraft_legal_guide`: Dog laws, taxes, mandatory courses, and insurance by region (`AT`, `Wien`, `DE`, `CH`).
- `puppycraft_toxic_check`: Check toxicity and danger level for foods, plants, or household items.
- `puppycraft_diary_list`: List recent dog diary entries ("Hundsi Tagebuch").
- `puppycraft_calendar_list`: List upcoming dog-sharing / sitter events ("Hundsi Verleih").
- `puppycraft_vaccine_list`: List vaccination records and due dates.
- `puppycraft_vet_list`: List veterinary visits and medications.
- `puppycraft_parks_list`: List saved dog parks (Hundezonen).
- `puppycraft_skill_read`: List or read bundled PuppyCraft skills.

## Write Tools (require user confirmation in chat)

- `puppycraft_diary_add`: Add a new entry to the dog diary (title, note, mood, tag, weight_kg).
- `puppycraft_calendar_add`: Schedule a dog-sharing / sitter handoff (sitter_name, relation, start_date, end_date, notes).
- `puppycraft_vaccine_add`: Record a vaccination or schedule a booster (vaccine_name, date_given, next_due).
- `puppycraft_vet_add`: Record a veterinary visit (clinic, reason, diagnosis_meds, cost_eur).

## Rules

- Always check `puppycraft_status` first when the user asks about "my dog" or upcoming dates.
- Quote tool results accurately; never invent breed scores, dates, or medical dosages.
