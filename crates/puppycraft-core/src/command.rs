use crate::encyclopedia::{all_breeds, find_breed, match_breeds};
use crate::error::PuppyCraftError;
use crate::knowledge::{
    calculate_chocolate_toxicity, legal_rules, toxic_substances, travel_destinations,
};
use crate::models::{LivingType, MatchProfile};
use crate::skills::{skill_read, skills_list};
use crate::store::PuppyStore;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub read_only: bool,
    pub input_schema: Value,
}

pub fn tools_list() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "puppycraft_status",
            description: "Overview of active dog profiles, diary count, upcoming sitter handoffs, and vaccinations.",
            read_only: true,
            input_schema: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolSpec {
            name: "puppycraft_breed_info",
            description: "Look up detailed dog breed info from the encyclopedia (or list all breeds if query omitted).",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Breed name or ID (e.g. 'Golden Retriever', 'Dackel', 'Pudel')" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_match_breeds",
            description: "Score and rank dog breeds for a household/location profile (presets: oldie, family_kids_cat, almoehi, city_single, sporty).",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "preset": { "type": "string", "description": "Optional preset: oldie | family_kids_cat | almoehi | city_single | sporty" },
                    "has_kids": { "type": "boolean" },
                    "has_cats": { "type": "boolean" },
                    "living": { "type": "string", "description": "city | suburban | rural" },
                    "daily_exercise_min": { "type": "integer" },
                    "limit": { "type": "integer", "description": "Max results (default 5)" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_travel_guide",
            description: "Get dog-friendliness, transit rules, and entry requirements for a city or country (e.g. Wien, Tokio, Deutschland, Italien, Kroatien).",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "destination": { "type": "string", "description": "City or country name (omit for all)" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_legal_guide",
            description: "Get dog laws, taxes, mandatory courses (Hundefuehrschein/Sachkunde), and insurance rules by region (Wien, NÖ, Deutschland, Schweiz).",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "region": { "type": "string", "description": "Optional region filter" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_toxic_check",
            description: "Check dog toxicity for a substance or calculate chocolate theobromine risk by dog weight and grams.",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "substance": { "type": "string", "description": "e.g. Schokolade, Weintrauben, Xylit, Zwiebeln" },
                    "dog_weight_kg": { "type": "number" },
                    "chocolate_grams": { "type": "number" },
                    "cocoa_type": { "type": "string", "description": "milk | dark | dark_70 | baking | white" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_diary_list",
            description: "List recent dog diary entries (Hundsi Tagebuch).",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "limit": { "type": "integer", "description": "Max entries (default 15)" }
                },
                "required": []
            }),
        },
        ToolSpec {
            name: "puppycraft_diary_add",
            description: "Add a new entry to the dog diary (Hundsi Tagebuch).",
            read_only: false,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dog_name": { "type": "string" },
                    "title": { "type": "string" },
                    "note": { "type": "string" },
                    "mood": { "type": "string" },
                    "tag": { "type": "string" },
                    "weight_kg": { "type": "number" }
                },
                "required": ["title", "note"]
            }),
        },
        ToolSpec {
            name: "puppycraft_calendar_list",
            description: "List scheduled dog-sharing / sitter events (Hundsi Verleih an Partner/Verwandte).",
            read_only: true,
            input_schema: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolSpec {
            name: "puppycraft_calendar_add",
            description: "Schedule a dog-sharing / sitter handoff with partner or relatives.",
            read_only: false,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dog_name": { "type": "string" },
                    "sitter_name": { "type": "string" },
                    "relation": { "type": "string", "description": "e.g. Partner, Oma, Schwester, Hundesitter" },
                    "start_date": { "type": "string", "description": "YYYY-MM-DD" },
                    "end_date": { "type": "string", "description": "YYYY-MM-DD" },
                    "feeding_notes": { "type": "string" },
                    "walk_notes": { "type": "string" },
                    "emergency_contact": { "type": "string" }
                },
                "required": ["sitter_name", "start_date", "end_date"]
            }),
        },
        ToolSpec {
            name: "puppycraft_vaccine_list",
            description: "List vaccination records and upcoming booster due dates.",
            read_only: true,
            input_schema: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolSpec {
            name: "puppycraft_vaccine_add",
            description: "Record a dog vaccination and its next due date.",
            read_only: false,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dog_name": { "type": "string" },
                    "vaccine_name": { "type": "string" },
                    "date_given": { "type": "string", "description": "YYYY-MM-DD" },
                    "next_due": { "type": "string", "description": "YYYY-MM-DD" },
                    "vet_clinic": { "type": "string" },
                    "notes": { "type": "string" }
                },
                "required": ["vaccine_name", "date_given", "next_due"]
            }),
        },
        ToolSpec {
            name: "puppycraft_vet_list",
            description: "List recorded veterinary visits and medications.",
            read_only: true,
            input_schema: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolSpec {
            name: "puppycraft_vet_add",
            description: "Record a veterinary visit, diagnosis/meds, and cost.",
            read_only: false,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "dog_name": { "type": "string" },
                    "date": { "type": "string", "description": "YYYY-MM-DD" },
                    "clinic": { "type": "string" },
                    "reason": { "type": "string" },
                    "diagnosis_meds": { "type": "string" },
                    "cost_eur": { "type": "number" }
                },
                "required": ["reason"]
            }),
        },
        ToolSpec {
            name: "puppycraft_parks_list",
            description: "List saved dog parks / Hundezonen with fenced and water info.",
            read_only: true,
            input_schema: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolSpec {
            name: "puppycraft_skill_read",
            description: "List bundled PuppyCraft skills or read a specific skill's SKILL.md body.",
            read_only: true,
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Skill name (omit to list all skills)" }
                },
                "required": []
            }),
        },
    ]
}

pub fn is_read_only_tool(name: &str) -> bool {
    tools_list()
        .iter()
        .find(|t| t.name == name)
        .is_some_and(|t| t.read_only)
}

pub fn dispatch_tool(
    store: &PuppyStore,
    name: &str,
    args: &Value,
) -> Result<Value, PuppyCraftError> {
    match name {
        "puppycraft_status" => {
            let dogs = store.list_dogs()?;
            let diary = store.list_diary(5)?;
            let events = store.list_sitter_events()?;
            let vaccines = store.list_vaccinations()?;
            let photos = store.list_photos()?;
            Ok(json!({
                "app": "puppycraft",
                "version": env!("CARGO_PKG_VERSION"),
                "breeds_count": all_breeds().len(),
                "dogs": dogs,
                "diary_recent_count": diary.len(),
                "sitter_events_count": events.len(),
                "upcoming_sitter_events": events.into_iter().take(5).collect::<Vec<_>>(),
                "vaccinations": vaccines,
                "photos_count": photos.len(),
            }))
        }
        "puppycraft_breed_info" => {
            let query = args.get("query").and_then(Value::as_str).unwrap_or("");
            if query.trim().is_empty() {
                let summary: Vec<Value> = all_breeds()
                    .into_iter()
                    .map(|b| json!({ "id": b.id, "name": b.name_de, "size": b.size, "energy": b.energy }))
                    .collect();
                Ok(json!({ "breeds": summary }))
            } else {
                let b = find_breed(query).ok_or_else(|| {
                    PuppyCraftError::NotFound(format!("breed not found: {query}"))
                })?;
                serde_json::to_value(b).map_err(PuppyCraftError::from)
            }
        }
        "puppycraft_match_breeds" => {
            let preset = args
                .get("preset")
                .and_then(Value::as_str)
                .unwrap_or("city_single");
            let mut profile = MatchProfile::preset(preset);
            if let Some(v) = args.get("has_kids").and_then(Value::as_bool) {
                profile.has_kids = v;
            }
            if let Some(v) = args.get("has_cats").and_then(Value::as_bool) {
                profile.has_cats = v;
            }
            if let Some(v) = args.get("living").and_then(Value::as_str) {
                profile.living = match v {
                    "rural" | "almoehi" => LivingType::RuralFarmAlmoehi,
                    "suburban" | "house" => LivingType::SuburbanHouse,
                    _ => LivingType::CityApartment,
                };
            }
            if let Some(v) = args.get("daily_exercise_min").and_then(Value::as_u64) {
                profile.daily_exercise_min = v as u16;
            }
            let limit = args
                .get("limit")
                .and_then(Value::as_u64)
                .unwrap_or(5)
                .clamp(1, 30) as usize;
            let matches: Vec<_> = match_breeds(&profile).into_iter().take(limit).collect();
            Ok(json!({ "profile": profile, "matches": matches }))
        }
        "puppycraft_travel_guide" => {
            let dest = args
                .get("destination")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_lowercase();
            let all = travel_destinations();
            if dest.is_empty() {
                Ok(json!({ "destinations": all }))
            } else {
                let filtered: Vec<_> = all
                    .into_iter()
                    .filter(|d| {
                        d.id.to_lowercase().contains(&dest)
                            || d.city_or_country.to_lowercase().contains(&dest)
                    })
                    .collect();
                Ok(json!({ "destinations": filtered }))
            }
        }
        "puppycraft_legal_guide" => {
            let region = args
                .get("region")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_lowercase();
            let rules: Vec<_> = legal_rules()
                .into_iter()
                .filter(|r| {
                    region.is_empty()
                        || r.region.to_lowercase().contains(&region)
                        || r.topic.to_lowercase().contains(&region)
                })
                .collect();
            Ok(json!({ "legal_rules": rules }))
        }
        "puppycraft_toxic_check" => {
            if let (Some(w), Some(g)) = (
                args.get("dog_weight_kg").and_then(Value::as_f64),
                args.get("chocolate_grams").and_then(Value::as_f64),
            ) {
                let cocoa = args
                    .get("cocoa_type")
                    .and_then(Value::as_str)
                    .unwrap_or("dark");
                let res = calculate_chocolate_toxicity(w as f32, g as f32, cocoa);
                return serde_json::to_value(res).map_err(PuppyCraftError::from);
            }
            let q = args
                .get("substance")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_lowercase();
            let list: Vec<_> = toxic_substances()
                .into_iter()
                .filter(|t| q.is_empty() || t.name.to_lowercase().contains(&q))
                .collect();
            Ok(json!({ "toxic_substances": list }))
        }
        "puppycraft_diary_list" => {
            let limit = args
                .get("limit")
                .and_then(Value::as_u64)
                .unwrap_or(15)
                .clamp(1, 200) as usize;
            let entries = store.list_diary(limit)?;
            Ok(json!({ "entries": entries }))
        }
        "puppycraft_diary_add" => {
            let dog = args
                .get("dog_name")
                .and_then(Value::as_str)
                .unwrap_or("Beppo");
            let title = args.get("title").and_then(Value::as_str).unwrap_or("");
            let note = args.get("note").and_then(Value::as_str).unwrap_or("");
            let mood = args
                .get("mood")
                .and_then(Value::as_str)
                .unwrap_or("Fröhlich");
            let tag = args.get("tag").and_then(Value::as_str).unwrap_or("Alltag");
            let weight = args
                .get("weight_kg")
                .and_then(Value::as_f64)
                .map(|v| v as f32);
            let today = Local::now().format("%Y-%m-%d").to_string();
            let id = store.add_diary(dog, &today, title, note, mood, tag, weight, None)?;
            Ok(json!({ "id": id, "date": today, "dog_name": dog, "title": title }))
        }
        "puppycraft_calendar_list" => {
            let events = store.list_sitter_events()?;
            Ok(json!({ "events": events }))
        }
        "puppycraft_calendar_add" => {
            let dog = args
                .get("dog_name")
                .and_then(Value::as_str)
                .unwrap_or("Beppo");
            let sitter = args
                .get("sitter_name")
                .and_then(Value::as_str)
                .unwrap_or("");
            let relation = args
                .get("relation")
                .and_then(Value::as_str)
                .unwrap_or("Familie");
            let start = args.get("start_date").and_then(Value::as_str).unwrap_or("");
            let end = args
                .get("end_date")
                .and_then(Value::as_str)
                .unwrap_or(start);
            let feeding = args
                .get("feeding_notes")
                .and_then(Value::as_str)
                .unwrap_or("");
            let walk = args.get("walk_notes").and_then(Value::as_str).unwrap_or("");
            let emergency = args
                .get("emergency_contact")
                .and_then(Value::as_str)
                .unwrap_or("");
            let id = store
                .add_sitter_event(dog, sitter, relation, start, end, feeding, walk, emergency)?;
            Ok(json!({ "id": id, "sitter_name": sitter, "start_date": start, "end_date": end }))
        }
        "puppycraft_vaccine_list" => {
            let list = store.list_vaccinations()?;
            Ok(json!({ "vaccinations": list }))
        }
        "puppycraft_vaccine_add" => {
            let dog = args
                .get("dog_name")
                .and_then(Value::as_str)
                .unwrap_or("Beppo");
            let vax = args
                .get("vaccine_name")
                .and_then(Value::as_str)
                .unwrap_or("");
            let given = args.get("date_given").and_then(Value::as_str).unwrap_or("");
            let due = args.get("next_due").and_then(Value::as_str).unwrap_or("");
            let clinic = args.get("vet_clinic").and_then(Value::as_str).unwrap_or("");
            let notes = args.get("notes").and_then(Value::as_str).unwrap_or("");
            let id = store.add_vaccination(dog, vax, given, due, clinic, notes)?;
            Ok(json!({ "id": id, "vaccine_name": vax, "next_due": due }))
        }
        "puppycraft_vet_list" => {
            let list = store.list_vet_visits()?;
            Ok(json!({ "vet_visits": list }))
        }
        "puppycraft_vet_add" => {
            let dog = args
                .get("dog_name")
                .and_then(Value::as_str)
                .unwrap_or("Beppo");
            let today = Local::now().format("%Y-%m-%d").to_string();
            let date = args.get("date").and_then(Value::as_str).unwrap_or(&today);
            let clinic = args.get("clinic").and_then(Value::as_str).unwrap_or("");
            let reason = args.get("reason").and_then(Value::as_str).unwrap_or("");
            let meds = args
                .get("diagnosis_meds")
                .and_then(Value::as_str)
                .unwrap_or("");
            let cost = args.get("cost_eur").and_then(Value::as_f64).unwrap_or(0.0) as f32;
            let id = store.add_vet_visit(dog, date, clinic, reason, meds, cost)?;
            Ok(json!({ "id": id, "date": date, "reason": reason }))
        }
        "puppycraft_parks_list" => {
            let parks = store.list_park_spots()?;
            Ok(json!({ "parks": parks }))
        }
        "puppycraft_skill_read" => {
            let sname = args
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim();
            if sname.is_empty() {
                let items: Vec<Value> = skills_list()
                    .iter()
                    .map(|s| json!({ "name": s.name, "description": s.description, "uri": format!("skill://{}/SKILL.md", s.name) }))
                    .collect();
                Ok(json!({ "skills": items }))
            } else {
                let s = skill_read(sname)?;
                Ok(json!({ "name": s.name, "description": s.description, "content": s.skill_md }))
            }
        }
        _ => Err(PuppyCraftError::NotFound(format!("unknown tool: {name}"))),
    }
}
