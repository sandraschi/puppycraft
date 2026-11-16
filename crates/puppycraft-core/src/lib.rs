pub mod command;
pub mod encyclopedia;
pub mod error;
pub mod knowledge;
pub mod llm;
pub mod models;
pub mod photo;
pub mod skills;
pub mod store;

#[cfg(test)]
mod tests {
    use crate::command::{dispatch_tool, tools_list};
    use crate::encyclopedia::{all_breeds, match_breeds};
    use crate::knowledge::calculate_chocolate_toxicity;
    use crate::models::MatchProfile;
    use crate::skills::{skill_by_uri, skills_list};
    use crate::store::PuppyStore;
    use serde_json::json;

    #[test]
    fn match_oldie_recommends_calm_small_dogs() {
        let p = MatchProfile::preset("oldie");
        let res = match_breeds(&p);
        assert!(!res.is_empty());
        assert!(res[0].score >= 80);
        // Border Collie should score significantly lower than Cavalier/Bichon/Havaneser for a lonely senior
        let border = res.iter().find(|r| r.breed.id == "border-collie");
        assert!(border.is_some());
        if let Some(bc) = border {
            assert!(bc.score < res[0].score);
        }
    }

    #[test]
    fn match_family_kids_cat_penalizes_high_prey_drive() {
        let p = MatchProfile::preset("family_kids_cat");
        let res = match_breeds(&p);
        let golden = res.iter().find(|r| r.breed.id == "golden-retriever");
        let jack = res.iter().find(|r| r.breed.id == "jack-russell");
        assert!(golden.is_some() && jack.is_some());
        if let (Some(g), Some(j)) = (golden, jack) {
            assert!(g.score > j.score);
        }
    }

    #[test]
    fn chocolate_calculator_detects_danger() {
        let safe = calculate_chocolate_toxicity(25.0, 10.0, "milk");
        assert!(safe.mg_per_kg < 5.0);
        let toxic = calculate_chocolate_toxicity(5.0, 100.0, "dark_70");
        assert!(toxic.mg_per_kg > 100.0);
    }

    #[test]
    fn store_roundtrip_diary_and_calendar() -> Result<(), Box<dyn std::error::Error>> {
        let store = PuppyStore::open_in_memory()?;
        let id = store.add_diary(
            "Beppo",
            "2026-11-15",
            "Parkrunde",
            "Brav gehört!",
            "Stolz",
            "Training",
            Some(6.9),
            None,
        )?;
        assert!(id > 0);
        let list = store.list_diary(10)?;
        assert!(list.iter().any(|e| e.title == "Parkrunde"));

        let cal_id = store.add_sitter_event(
            "Beppo",
            "Tante Mia",
            "Verwandte",
            "2026-12-01",
            "2026-12-03",
            "Trockenfutter",
            "2x Gassi",
            "Dr. Muster",
        )?;
        assert!(cal_id > 0);
        Ok(())
    }

    #[test]
    fn bundled_skills_meet_standard_and_tool_refs_exist() {
        let skills = skills_list();
        assert_eq!(skills.len(), 2);
        let tool_names: Vec<&str> = tools_list().into_iter().map(|t| t.name).collect();
        let mut total_bytes = 0usize;

        for s in skills {
            total_bytes += s.skill_md.len();
            assert!(s.skill_md.len() <= 12_288);
            assert!(s.description.len() <= 200);
            assert!(s.skill_md.contains(&format!("name: {}", s.name)));
            let by_uri = skill_by_uri(&format!("skill://{}/SKILL.md", s.name));
            assert!(by_uri.is_ok());

            // Every `puppycraft_*` in backticks must be a real tool
            for part in s.skill_md.split('`') {
                if part.starts_with("puppycraft_") {
                    assert!(
                        tool_names.contains(&part),
                        "unknown tool referenced in skill {}: {}",
                        s.name,
                        part
                    );
                }
            }
        }
        assert!(total_bytes <= 40_960);
    }

    #[test]
    fn dispatch_status_and_travel() -> Result<(), Box<dyn std::error::Error>> {
        let store = PuppyStore::open_in_memory()?;
        let st = dispatch_tool(&store, "puppycraft_status", &json!({}))?;
        assert_eq!(st["breeds_count"], all_breeds().len());

        let tokio = dispatch_tool(
            &store,
            "puppycraft_travel_guide",
            &json!({ "destination": "Tokio" }),
        )?;
        let arr = tokio["destinations"]
            .as_array()
            .ok_or("missing destinations array")?;
        assert_eq!(arr.len(), 1);
        assert!(arr[0]["dog_friendliness_score"].as_u64().unwrap_or(100) < 30);
        Ok(())
    }

    #[test]
    fn store_vet_visits_parks_and_sitter_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
        let store = PuppyStore::open_in_memory()?;
        let vid = store.add_vet_visit(
            "Beppo",
            "2026-11-15",
            "Tierklinik Wien",
            "Jahrescheck",
            "Alles top, 6.8 kg",
            65.0,
        )?;
        assert!(vid > 0);
        assert_eq!(store.list_vet_visits()?.len(), 1);

        let pid = store.add_park_spot(
            "Wienerberg Hundezone",
            "Wien 10",
            true,
            true,
            5,
            "Viel Platz",
        )?;
        assert!(pid > 0);
        assert!(store.list_park_spots()?.len() >= 3);

        let sid = store.add_sitter_event(
            "Beppo",
            "Opa Franz",
            "Verwandter",
            "2026-12-10",
            "2026-12-11",
            "180g",
            "2x",
            "Dr. Muster",
        )?;
        store.update_sitter_status(sid, "Bestätigt")?;
        let evs = store.list_sitter_events()?;
        assert!(evs.iter().any(|e| e.id == sid && e.status == "Bestätigt"));
        store.delete_sitter_event(sid)?;
        let evs2 = store.list_sitter_events()?;
        assert!(!evs2.iter().any(|e| e.id == sid));
        Ok(())
    }

    #[test]
    fn every_skill_folder_is_bundled() -> Result<(), Box<dyn std::error::Error>> {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let skills_dir = manifest_dir.join("..").join("..").join("skills");
        let bundled: Vec<&str> = skills_list().iter().map(|s| s.name).collect();
        for entry in std::fs::read_dir(&skills_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                assert!(
                    bundled.contains(&name.as_str()),
                    "skill folder '{name}' is missing from SKILLS bundle"
                );
            }
        }
        Ok(())
    }
}
