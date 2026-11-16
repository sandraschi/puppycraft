use chrono::Local;
use egui::{Color32, ColorImage, RichText, ScrollArea, TextureHandle, TextureOptions, Ui};
use image::DynamicImage;
use puppycraft_core::knowledge::{
    body_language_signals, calculate_chocolate_toxicity, calculate_daily_food, calculate_tco,
    daily_routine, dog_names, first_aid_guides, insurance_checklist, quiz_questions,
    senior_care_tips, separation_training_plan, starter_kit_items, toxic_substances,
};
use puppycraft_core::models::DogSize;
use puppycraft_core::photo::{apply_edits, load_image, save_edited_image, PhotoEditParams};
use puppycraft_core::store::PuppyStore;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraToolTab {
    ToxicAndChoco,
    FoodCalc,
    CostCalc,
    StarterKit,
    BodyLanguage,
    FirstAid,
    NameFinder,
    Quiz,
    SeparationAndRoutine,
    ParksAndVet,
    InsuranceAndSenior,
}

pub struct LifeState {
    // Diary
    pub diary_dog: String,
    pub diary_title: String,
    pub diary_note: String,
    pub diary_mood: String,
    pub diary_tag: String,
    pub diary_weight: String,
    pub diary_msg: String,

    // Sitter Calendar
    pub sit_dog: String,
    pub sit_name: String,
    pub sit_relation: String,
    pub sit_start: String,
    pub sit_end: String,
    pub sit_feeding: String,
    pub sit_walk: String,
    pub sit_emergency: String,
    pub sit_msg: String,
    pub handoff_preview: Option<String>,

    // Photo tool
    pub photo_src_path: Option<PathBuf>,
    pub photo_raw: Option<DynamicImage>,
    pub photo_params: PhotoEditParams,
    pub photo_texture: Option<TextureHandle>,
    pub photo_caption: String,
    pub photo_msg: String,

    // Extra tools
    pub extra_tab: ExtraToolTab,
    pub choco_weight_kg: f32,
    pub choco_grams: f32,
    pub choco_type: String,
    pub food_weight_kg: f32,
    pub food_stage: String,
    pub food_activity: String,
    pub tco_size: DogSize,
    pub tco_insurance: bool,
    pub quiz_answers: [Option<usize>; 5],
    pub routine_stage: String,

    // Parks & Vet forms
    pub park_name: String,
    pub park_city: String,
    pub park_fenced: bool,
    pub park_water: bool,
    pub park_notes: String,
    pub vet_clinic: String,
    pub vet_reason: String,
    pub vet_meds: String,
    pub vet_cost: String,
}

impl Default for LifeState {
    fn default() -> Self {
        let today = Local::now().format("%Y-%m-%d").to_string();
        Self {
            diary_dog: "Beppo".into(),
            diary_title: String::new(),
            diary_note: String::new(),
            diary_mood: "Fröhlich".into(),
            diary_tag: "Alltag".into(),
            diary_weight: String::new(),
            diary_msg: String::new(),

            sit_dog: "Beppo".into(),
            sit_name: String::new(),
            sit_relation: "Partner / Verwandte".into(),
            sit_start: today.clone(),
            sit_end: today,
            sit_feeding: "180g morgens & abends, frisches Wasser".into(),
            sit_walk: "3x täglich Gassi, im Park Schleppleine".into(),
            sit_emergency: "Tierarzt +43 1 555 0199".into(),
            sit_msg: String::new(),
            handoff_preview: None,

            photo_src_path: None,
            photo_raw: None,
            photo_params: PhotoEditParams::default(),
            photo_texture: None,
            photo_caption: "Mein Hundsi".into(),
            photo_msg: String::new(),

            extra_tab: ExtraToolTab::ToxicAndChoco,
            choco_weight_kg: 10.0,
            choco_grams: 30.0,
            choco_type: "dark_70".into(),
            food_weight_kg: 10.0,
            food_stage: "adult".into(),
            food_activity: "normal".into(),
            tco_size: DogSize::Medium,
            tco_insurance: true,
            quiz_answers: [None; 5],
            routine_stage: "puppy".into(),
            park_name: String::new(),
            park_city: "Wien".into(),
            park_fenced: true,
            park_water: true,
            park_notes: String::new(),
            vet_clinic: "Tierklinik Wien".into(),
            vet_reason: String::new(),
            vet_meds: String::new(),
            vet_cost: "50".into(),
        }
    }
}

pub fn ui_diary(ui: &mut Ui, st: &mut LifeState, store: &Arc<PuppyStore>) {
    ui.heading("Hundsi-Tagebuch (Meilensteine, Gewicht & Alltag)");

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(RichText::new("Neuer Tagebuch-Eintrag").strong());
        egui::Grid::new("diary_form").num_columns(2).show(ui, |ui| {
            ui.label("Hund:");
            ui.text_edit_singleline(&mut st.diary_dog);
            ui.end_row();
            ui.label("Titel:");
            ui.text_edit_singleline(&mut st.diary_title);
            ui.end_row();
            ui.label("Stimmung / Tag:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut st.diary_mood);
                ui.text_edit_singleline(&mut st.diary_tag);
            });
            ui.end_row();
            ui.label("Gewicht (kg, optional):");
            ui.text_edit_singleline(&mut st.diary_weight);
            ui.end_row();
            ui.label("Notiz:");
            ui.text_edit_multiline(&mut st.diary_note);
            ui.end_row();
        });
        if ui.button("Eintrag speichern").clicked() {
            let today = Local::now().format("%Y-%m-%d").to_string();
            let weight = st.diary_weight.trim().replace(',', ".").parse::<f32>().ok();
            match store.add_diary(
                &st.diary_dog,
                &today,
                &st.diary_title,
                &st.diary_note,
                &st.diary_mood,
                &st.diary_tag,
                weight,
                None,
            ) {
                Ok(_) => {
                    st.diary_msg = "Tagebuch-Eintrag gespeichert!".into();
                    st.diary_title.clear();
                    st.diary_note.clear();
                }
                Err(e) => st.diary_msg = format!("Fehler: {e}"),
            }
        }
        if !st.diary_msg.is_empty() {
            ui.label(&st.diary_msg);
        }
    });

    ui.separator();
    if let Ok(entries) = store.list_diary(50) {
        let weights: Vec<(String, f32)> = entries
            .iter()
            .rev()
            .filter_map(|e| e.weight_kg.map(|w| (e.date.clone(), w)))
            .collect();
        if weights.len() >= 2 {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new("Gewichtskurve (kg)").strong());
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width().min(500.0), 64.0),
                    egui::Sense::hover(),
                );
                let min_w = weights.iter().map(|x| x.1).fold(f32::INFINITY, f32::min) - 0.5;
                let max_w = weights
                    .iter()
                    .map(|x| x.1)
                    .fold(f32::NEG_INFINITY, f32::max)
                    + 0.5;
                let span = (max_w - min_w).max(0.5);
                let pts: Vec<egui::Pos2> = weights
                    .iter()
                    .enumerate()
                    .map(|(i, (_, w))| {
                        let t = i as f32 / (weights.len() - 1) as f32;
                        let x = rect.left() + 8.0 + t * (rect.width() - 16.0);
                        let y =
                            rect.bottom() - 8.0 - ((*w - min_w) / span) * (rect.height() - 16.0);
                        egui::pos2(x, y)
                    })
                    .collect();
                let painter = ui.painter();
                painter.add(egui::Shape::line(
                    pts.clone(),
                    egui::Stroke::new(2.0, Color32::from_rgb(80, 185, 130)),
                ));
                for pt in pts {
                    painter.circle_filled(pt, 3.5, Color32::from_rgb(240, 190, 80));
                }
            });
        }
        ScrollArea::vertical().show(ui, |ui| {
            for e in entries {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&e.date).weak());
                        ui.label(RichText::new(&e.title).strong());
                        ui.label(format!("[{} | {}]", e.tag, e.mood));
                        if let Some(w) = e.weight_kg {
                            ui.label(format!("({w:.1} kg)"));
                        }
                    });
                    ui.label(&e.note);
                });
            }
        });
    }
}

pub fn ui_calendar(ui: &mut Ui, st: &mut LifeState, store: &Arc<PuppyStore>) {
    ui.heading("Kalender für Hundsi-Verleih (Partner, Oma & Verwandte)");
    ui.label("Plane Betreuungszeiten und generiere mit einem Klick einen Übergabe-Steckbrief.");

    egui::Frame::group(ui.style()).show(ui, |ui| {
        egui::Grid::new("cal_form").num_columns(2).show(ui, |ui| {
            ui.label("Hund:");
            ui.text_edit_singleline(&mut st.sit_dog);
            ui.end_row();
            ui.label("Betreuer*in (Name):");
            ui.text_edit_singleline(&mut st.sit_name);
            ui.end_row();
            ui.label("Beziehung:");
            ui.text_edit_singleline(&mut st.sit_relation);
            ui.end_row();
            ui.label("Von / Bis (YYYY-MM-DD):");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut st.sit_start);
                ui.label("bis");
                ui.text_edit_singleline(&mut st.sit_end);
            });
            ui.end_row();
            ui.label("Fütterung:");
            ui.text_edit_singleline(&mut st.sit_feeding);
            ui.end_row();
            ui.label("Gassi-Hinweise:");
            ui.text_edit_singleline(&mut st.sit_walk);
            ui.end_row();
            ui.label("Notfall-Kontakt:");
            ui.text_edit_singleline(&mut st.sit_emergency);
            ui.end_row();
        });
        if ui.button("Betreuung eintragen").clicked() {
            match store.add_sitter_event(
                &st.sit_dog,
                &st.sit_name,
                &st.sit_relation,
                &st.sit_start,
                &st.sit_end,
                &st.sit_feeding,
                &st.sit_walk,
                &st.sit_emergency,
            ) {
                Ok(_) => {
                    st.sit_msg = "Betreuung eingetragen!".into();
                    st.sit_name.clear();
                }
                Err(e) => st.sit_msg = format!("Fehler: {e}"),
            }
        }
        if !st.sit_msg.is_empty() {
            ui.label(&st.sit_msg);
        }
    });

    ui.separator();
    ScrollArea::vertical().show(ui, |ui| {
        if let Ok(events) = store.list_sitter_events() {
            for ev in events {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{} -> {} | {} ({}) [{}]",
                                ev.start_date, ev.end_date, ev.sitter_name, ev.relation, ev.status
                            ))
                            .strong(),
                        );
                        if ui.small_button("Bestätigen").clicked() {
                            let _ = store.update_sitter_status(ev.id, "Bestätigt");
                        }
                        if ui.small_button("Erledigt").clicked() {
                            let _ = store.update_sitter_status(ev.id, "Abgeschlossen");
                        }
                        if ui.small_button("Löschen").clicked() {
                            let _ = store.delete_sitter_event(ev.id);
                        }
                        if ui.small_button("Übergabe-Steckbrief").clicked() {
                            st.handoff_preview = Some(format!(
                                "# Hundsi Übergabe-Steckbrief: {}\n\n- **Betreuer*in:** {} ({})\n- **Zeitraum:** {} bis {}\n- **Fütterung:** {}\n- **Gassi & Besonderheiten:** {}\n- **Tierarzt / Notfall:** {}\n\n*Hinweis: Keine Weintrauben, keine Schokolade, kein Xylit füttern!*",
                                ev.dog_name,
                                ev.sitter_name,
                                ev.relation,
                                ev.start_date,
                                ev.end_date,
                                ev.feeding_notes,
                                ev.walk_notes,
                                ev.emergency_contact
                            ));
                        }
                    });
                    ui.label(format!("Futter: {}", ev.feeding_notes));
                    ui.label(format!("Gassi: {}", ev.walk_notes));
                    ui.label(RichText::new(format!("Notfall: {}", ev.emergency_contact)).weak());
                });
            }
        }
    });

    if let Some(txt) = st.handoff_preview.clone() {
        let mut open = true;
        egui::Window::new("Übergabe-Steckbrief für Partner/Verwandte")
            .open(&mut open)
            .default_width(520.0)
            .show(ui.ctx(), |ui| {
                ui.monospace(&txt);
                if ui.button("In Zwischenablage kopieren").clicked() {
                    ui.ctx().copy_text(txt.clone());
                }
            });
        if !open {
            st.handoff_preview = None;
        }
    }
}

fn refresh_photo_texture(ctx: &egui::Context, st: &mut LifeState) {
    let Some(raw) = st.photo_raw.as_ref() else {
        return;
    };
    let edited = apply_edits(raw, &st.photo_params);
    let rgba = edited.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let color_img = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    st.photo_texture = Some(ctx.load_texture("hundsi_photo", color_img, TextureOptions::LINEAR));
}

pub fn ui_photo_tool(ui: &mut Ui, st: &mut LifeState, store: &Arc<PuppyStore>) {
    ui.heading("Phototool für Hundsiphotos");
    ui.label(
        "Lade ein Foto von deinem Hund, passe Licht/Wärme/Drehung/Zuschnitt an und speichere es.",
    );

    let mut dirty = false;
    ui.horizontal_wrapped(|ui| {
        if ui.button("Foto öffnen...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Bilder", &["png", "jpg", "jpeg", "webp", "bmp"])
                .pick_file()
            {
                match load_image(&path) {
                    Ok(img) => {
                        st.photo_src_path = Some(path);
                        st.photo_raw = Some(img);
                        st.photo_params = PhotoEditParams::default();
                        dirty = true;
                        st.photo_msg = "Foto geladen.".into();
                    }
                    Err(e) => st.photo_msg = format!("Fehler beim Laden: {e}"),
                }
            }
        }
        if ui.button("90 Grad drehen").clicked() {
            st.photo_params.rotate_deg = (st.photo_params.rotate_deg + 90) % 360;
            dirty = true;
        }
        if ui.button("Zurücksetzen").clicked() {
            st.photo_params = PhotoEditParams::default();
            dirty = true;
        }
        if st.photo_raw.is_some() && ui.button("Bearbeitetes Foto speichern...").clicked() {
            if let Some(dest) = rfd::FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .set_file_name("hundsi_bearbeitet.png")
                .save_file()
            {
                if let Some(raw) = st.photo_raw.as_ref() {
                    match save_edited_image(raw, &st.photo_params, &dest) {
                        Ok(()) => {
                            let today = Local::now().format("%Y-%m-%d").to_string();
                            let _ = store.add_photo(
                                &st.diary_dog,
                                &dest.display().to_string(),
                                &st.photo_caption,
                                &today,
                            );
                            st.photo_msg = format!("Gespeichert: {}", dest.display());
                        }
                        Err(e) => st.photo_msg = format!("Speicherfehler: {e}"),
                    }
                }
            }
        }
    });

    ui.horizontal_wrapped(|ui| {
        if ui
            .add(egui::Slider::new(&mut st.photo_params.brightness, -50..=50).text("Helligkeit"))
            .changed()
        {
            dirty = true;
        }
        if ui
            .add(egui::Slider::new(&mut st.photo_params.contrast, -40.0..=40.0).text("Kontrast"))
            .changed()
        {
            dirty = true;
        }
        if ui
            .add(
                egui::Slider::new(&mut st.photo_params.warmth, -30..=30)
                    .text("Wärme (Golden Glow)"),
            )
            .changed()
        {
            dirty = true;
        }
        if ui
            .add(egui::Slider::new(&mut st.photo_params.crop_scale, 0.4..=1.0).text("Zoom/Crop"))
            .changed()
        {
            dirty = true;
        }
        if ui
            .checkbox(&mut st.photo_params.polaroid_frame, "Polaroid-Rahmen")
            .changed()
        {
            dirty = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Bildunterschrift:");
        ui.text_edit_singleline(&mut st.photo_caption);
    });

    if dirty {
        refresh_photo_texture(ui.ctx(), st);
    }

    if !st.photo_msg.is_empty() {
        ui.label(&st.photo_msg);
    }

    ui.separator();
    if let Some(tex) = st.photo_texture.as_ref() {
        let avail = ui.available_size();
        let tex_size = tex.size_vec2();
        let scale = (avail.x / tex_size.x)
            .min((avail.y - 110.0).max(120.0) / tex_size.y)
            .min(1.0);
        ui.image((tex.id(), tex_size * scale));
    } else {
        ui.label(
            RichText::new("Noch kein Foto geöffnet. Klicke oben auf 'Foto öffnen...'.").weak(),
        );
    }

    if let Ok(photos) = store.list_photos() {
        if !photos.is_empty() {
            ui.separator();
            ui.label(RichText::new("Gespeicherte Hundsi-Galerie").strong());
            for p in photos.iter().take(5) {
                ui.horizontal(|ui| {
                    ui.label(format!("{} — {} ({})", p.date, p.caption, p.file_path));
                    if ui.small_button("Laden").clicked() {
                        let pb = PathBuf::from(&p.file_path);
                        if let Ok(img) = load_image(&pb) {
                            st.photo_src_path = Some(pb);
                            st.photo_raw = Some(img);
                            st.photo_params = PhotoEditParams::default();
                            refresh_photo_texture(ui.ctx(), st);
                        }
                    }
                });
            }
        }
    }
}

pub fn ui_extra_tools(ui: &mut Ui, st: &mut LifeState, store: &Arc<PuppyStore>) {
    ui.heading("Mehr Hundsi-Werkzeuge (20+ Features für Alltag & Vorbereitung)");
    ui.horizontal_wrapped(|ui| {
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::ToxicAndChoco,
            "Gift-Lexikon & Schoko",
        );
        ui.selectable_value(&mut st.extra_tab, ExtraToolTab::FoodCalc, "Futter-Rechner");
        ui.selectable_value(&mut st.extra_tab, ExtraToolTab::CostCalc, "Kosten-Rechner");
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::StarterKit,
            "Erstausstattung",
        );
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::BodyLanguage,
            "Körpersprache",
        );
        ui.selectable_value(&mut st.extra_tab, ExtraToolTab::FirstAid, "Erste Hilfe");
        ui.selectable_value(&mut st.extra_tab, ExtraToolTab::NameFinder, "Namen-Finder");
        ui.selectable_value(&mut st.extra_tab, ExtraToolTab::Quiz, "Sachkunde-Quiz");
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::SeparationAndRoutine,
            "Alleinbleiben & Routine",
        );
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::ParksAndVet,
            "Hundezonen & Tierarzt",
        );
        ui.selectable_value(
            &mut st.extra_tab,
            ExtraToolTab::InsuranceAndSenior,
            "Versicherung & Senioren",
        );
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| match st.extra_tab {
        ExtraToolTab::ToxicAndChoco => {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(RichText::new("Schokoladen-Notfallrechner (Theobromin)").strong());
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::Slider::new(&mut st.choco_weight_kg, 1.5..=60.0).text("Hund kg"));
                    ui.add(
                        egui::Slider::new(&mut st.choco_grams, 1.0..=300.0).text("Schokolade g"),
                    );
                    ui.selectable_value(&mut st.choco_type, "milk".into(), "Vollmilch");
                    ui.selectable_value(&mut st.choco_type, "dark".into(), "Dunkel (~50%)");
                    ui.selectable_value(&mut st.choco_type, "dark_70".into(), "Zartbitter 70%+");
                    ui.selectable_value(&mut st.choco_type, "baking".into(), "Backkakao");
                });
                let res = calculate_chocolate_toxicity(
                    st.choco_weight_kg,
                    st.choco_grams,
                    &st.choco_type,
                );
                let col = if res.mg_per_kg >= 40.0 {
                    Color32::from_rgb(225, 70, 70)
                } else if res.mg_per_kg >= 15.0 {
                    Color32::from_rgb(225, 165, 55)
                } else {
                    Color32::from_rgb(80, 190, 110)
                };
                ui.colored_label(
                    col,
                    format!(
                        "{} — {:.1} mg Theobromin gesamt ({:.1} mg/kg)",
                        res.severity, res.total_theobromine_mg, res.mg_per_kg
                    ),
                );
                ui.label(&res.advice);
            });
            ui.add_space(6.0);
            for t in toxic_substances() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(format!("{} — {}", t.name, t.danger_level)).strong());
                    ui.label(format!("Dosis: {}", t.toxic_dose));
                    ui.label(format!("Symptome: {}", t.symptoms));
                    ui.colored_label(Color32::from_rgb(220, 130, 60), t.action);
                });
            }
        }
        ExtraToolTab::FoodCalc => {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Slider::new(&mut st.food_weight_kg, 1.5..=70.0).text("Gewicht kg"));
                ui.selectable_value(&mut st.food_stage, "puppy_young".into(), "Welpe (bis 4 Mo)");
                ui.selectable_value(
                    &mut st.food_stage,
                    "puppy_older".into(),
                    "Junghund (4-12 Mo)",
                );
                ui.selectable_value(&mut st.food_stage, "adult".into(), "Erwachsen");
                ui.selectable_value(&mut st.food_stage, "senior".into(), "Senior");
                ui.selectable_value(&mut st.food_activity, "low".into(), "Ruhig");
                ui.selectable_value(&mut st.food_activity, "normal".into(), "Normal");
                ui.selectable_value(&mut st.food_activity, "high".into(), "Sportlich");
            });
            let (_kcal, _dry, note) =
                calculate_daily_food(st.food_weight_kg, &st.food_stage, &st.food_activity);
            ui.label(RichText::new(note).strong().size(15.0));
        }
        ExtraToolTab::CostCalc => {
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut st.tco_size, DogSize::Small, "Klein");
                ui.selectable_value(&mut st.tco_size, DogSize::Medium, "Mittel");
                ui.selectable_value(&mut st.tco_size, DogSize::Large, "Groß");
                ui.selectable_value(&mut st.tco_size, DogSize::Giant, "Riese");
                ui.checkbox(&mut st.tco_insurance, "Inkl. OP-/Krankenversicherung");
            });
            let (setup, monthly, buf) = calculate_tco(st.tco_size, st.tco_insurance);
            ui.label(format!("Erstausstattung (einmalig): ~{setup} EUR"));
            ui.label(format!(
                "Laufende Kosten pro Monat: ~{monthly} EUR (Futter, Steuer, Haftpflicht, Rücklage)"
            ));
            ui.label(format!(
                "Empfohlener Notfall-Puffer (Tierklinik): {buf} EUR"
            ));
        }
        ExtraToolTab::StarterKit => {
            for (item, why) in starter_kit_items() {
                ui.label(format!("• {item} — {why}"));
            }
        }
        ExtraToolTab::BodyLanguage => {
            for (sig, meaning) in body_language_signals() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(sig).strong());
                    ui.label(meaning);
                });
            }
        }
        ExtraToolTab::FirstAid => {
            for (title, guide) in first_aid_guides() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(
                        RichText::new(title)
                            .strong()
                            .color(Color32::from_rgb(225, 110, 70)),
                    );
                    ui.label(guide);
                });
            }
        }
        ExtraToolTab::NameFinder => {
            for (name, style, desc) in dog_names() {
                ui.label(format!("• {name} [{style}] — {desc}"));
            }
        }
        ExtraToolTab::Quiz => {
            let qs = quiz_questions();
            for (qi, (q, opts, correct, expl)) in qs.iter().enumerate() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(format!("{}. {q}", qi + 1)).strong());
                    for (oi, opt) in opts.iter().enumerate() {
                        let chosen = st.quiz_answers[qi] == Some(oi);
                        if ui.radio(chosen, *opt).clicked() {
                            st.quiz_answers[qi] = Some(oi);
                        }
                    }
                    if let Some(ans) = st.quiz_answers[qi] {
                        if ans == *correct {
                            ui.colored_label(
                                Color32::from_rgb(80, 190, 110),
                                format!("Richtig! {expl}"),
                            );
                        } else {
                            ui.colored_label(
                                Color32::from_rgb(220, 90, 80),
                                format!("Nicht ganz: {expl}"),
                            );
                        }
                    }
                });
            }
        }
        ExtraToolTab::SeparationAndRoutine => {
            ui.label(
                RichText::new("Alleinbleib-Trainingsplan (5 Stufen)")
                    .strong()
                    .size(15.0),
            );
            for (step, desc) in separation_training_plan() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(step).strong());
                    ui.label(desc);
                });
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new("Tagesroutine:").strong());
                ui.selectable_value(
                    &mut st.routine_stage,
                    "puppy".into(),
                    "Welpe (18-20h Schlaf)",
                );
                ui.selectable_value(&mut st.routine_stage, "adult".into(), "Erwachsen");
                ui.selectable_value(&mut st.routine_stage, "senior".into(), "Senior");
            });
            for (time, act) in daily_routine(&st.routine_stage) {
                ui.label(format!("• {time} — {act}"));
            }
        }
        ExtraToolTab::ParksAndVet => {
            ui.label(
                RichText::new("Hundezonen & Auslauf-Merkliste")
                    .strong()
                    .size(15.0),
            );
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut st.park_name);
                    ui.label("Ort:");
                    ui.text_edit_singleline(&mut st.park_city);
                    ui.checkbox(&mut st.park_fenced, "Eingezäunt");
                    ui.checkbox(&mut st.park_water, "Wasser");
                });
                ui.horizontal(|ui| {
                    ui.label("Notiz:");
                    ui.text_edit_singleline(&mut st.park_notes);
                    if ui.button("Hundezone speichern").clicked()
                        && store
                            .add_park_spot(
                                &st.park_name,
                                &st.park_city,
                                st.park_fenced,
                                st.park_water,
                                5,
                                &st.park_notes,
                            )
                            .is_ok()
                    {
                        st.park_name.clear();
                        st.park_notes.clear();
                    }
                });
            });
            if let Ok(parks) = store.list_park_spots() {
                for p in parks {
                    ui.label(format!(
                        "• {} ({}) [{}{}] — {}",
                        p.name,
                        p.city,
                        if p.fenced { "eingezäunt" } else { "offen" },
                        if p.water_nearby { ", Wasser" } else { "" },
                        p.notes
                    ));
                }
            }

            ui.separator();
            ui.label(
                RichText::new("Tierarzt- & Medikamenten-Akte")
                    .strong()
                    .size(15.0),
            );
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Klinik:");
                    ui.text_edit_singleline(&mut st.vet_clinic);
                    ui.label("Grund:");
                    ui.text_edit_singleline(&mut st.vet_reason);
                    ui.label("Befund/Meds:");
                    ui.text_edit_singleline(&mut st.vet_meds);
                    ui.label("EUR:");
                    ui.text_edit_singleline(&mut st.vet_cost);
                    if ui.button("Besuch speichern").clicked() {
                        let today = Local::now().format("%Y-%m-%d").to_string();
                        let cost = st
                            .vet_cost
                            .trim()
                            .replace(',', ".")
                            .parse::<f32>()
                            .unwrap_or(0.0);
                        if store
                            .add_vet_visit(
                                &st.diary_dog,
                                &today,
                                &st.vet_clinic,
                                &st.vet_reason,
                                &st.vet_meds,
                                cost,
                            )
                            .is_ok()
                        {
                            st.vet_reason.clear();
                            st.vet_meds.clear();
                        }
                    }
                });
            });
            if let Ok(visits) = store.list_vet_visits() {
                for v in visits {
                    ui.label(format!(
                        "• {} | {} ({}) — {} | {:.2} EUR",
                        v.date, v.reason, v.clinic, v.diagnosis_meds, v.cost_eur
                    ));
                }
            }
        }
        ExtraToolTab::InsuranceAndSenior => {
            ui.label(
                RichText::new("Versicherungs-Checkliste")
                    .strong()
                    .size(15.0),
            );
            for (title, desc) in insurance_checklist() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(title).strong());
                    ui.label(desc);
                });
            }
            ui.add_space(6.0);
            ui.label(
                RichText::new("Seniorenhund-Pflege (für Oldies)")
                    .strong()
                    .size(15.0),
            );
            for (title, desc) in senior_care_tips() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(title).strong());
                    ui.label(desc);
                });
            }
        }
    });
}
