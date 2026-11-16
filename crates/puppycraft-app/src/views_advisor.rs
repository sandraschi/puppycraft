use crate::theme::{card_frame, draw_breed_portrait, hero_header, palette, pill_badge, stat_bar};
use egui::{Color32, RichText, ScrollArea, Ui};
use puppycraft_core::encyclopedia::{all_breeds, match_breeds};
use puppycraft_core::knowledge::{
    breeder_red_flags, breeders, rescue_foreign_checklist, rescue_orgs,
};
use puppycraft_core::models::{Breed, DogSize, LivingType, MatchProfile};

pub struct AdvisorState {
    pub profile: MatchProfile,
    pub breed_query: String,
    pub selected_breed_id: String,
    pub compare_mode: bool,
    pub compare_a: String,
    pub compare_b: String,
    pub show_rescue_tab: bool,
}

impl Default for AdvisorState {
    fn default() -> Self {
        Self {
            profile: MatchProfile::preset("family_kids_cat"),
            breed_query: String::new(),
            selected_breed_id: "golden-retriever".into(),
            compare_mode: false,
            compare_a: "golden-retriever".into(),
            compare_b: "cavalier-king-charles".into(),
            show_rescue_tab: false,
        }
    }
}

pub fn ui_matcher(ui: &mut Ui, st: &mut AdvisorState) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Welcher Hund passt zu dir?",
        "Familien-, Wohnort- & Alltagsmatching (von Einsamer Oldie bis Almöhi)",
    );

    ui.horizontal_wrapped(|ui| {
        if ui.button("Einsamer Oldie").clicked() {
            st.profile = MatchProfile::preset("oldie");
        }
        if ui.button("Familie mit 3 Kids & Katze").clicked() {
            st.profile = MatchProfile::preset("family_kids_cat");
        }
        if ui.button("Almöhi / Bergbauer").clicked() {
            st.profile = MatchProfile::preset("almoehi");
        }
        if ui.button("Innenstadt-Single").clicked() {
            st.profile = MatchProfile::preset("city_single");
        }
        if ui.button("Sportler / Outdoor").clicked() {
            st.profile = MatchProfile::preset("sporty");
        }
    });

    ui.separator();
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Wohnort:").strong());
            ui.selectable_value(
                &mut st.profile.living,
                LivingType::CityApartment,
                "Innenstadt-Wohnung",
            );
            ui.selectable_value(
                &mut st.profile.living,
                LivingType::SuburbanHouse,
                "Haus mit Garten",
            );
            ui.selectable_value(
                &mut st.profile.living,
                LivingType::RuralFarmAlmoehi,
                "Almöhi / Alleinlage",
            );
        });

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut st.profile.has_kids, "Kinder im Haushalt");
            if st.profile.has_kids {
                ui.add(egui::Slider::new(&mut st.profile.kids_count, 1..=5).text("Anzahl Kids"));
            }
            ui.checkbox(&mut st.profile.has_cats, "Katze(n) vorhanden");
            ui.checkbox(&mut st.profile.is_senior_owner, "Senior / Oldie");
            ui.checkbox(&mut st.profile.first_time_owner, "Ersthundebesitzer");
        });

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut st.profile.has_elevator_or_ground, "Lift / Erdgeschoss");
            ui.checkbox(
                &mut st.profile.quiet_neighbor_needed,
                "Ruhige Nachbarn nötig",
            );
            ui.checkbox(
                &mut st.profile.low_shedding_preferred,
                "Wenig Haaren / Allergiker",
            );
        });

        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::Slider::new(&mut st.profile.daily_exercise_min, 20..=180)
                    .text("Min. Auslauf/Tag"),
            );
            ui.add(
                egui::Slider::new(&mut st.profile.alone_hours_day, 0.0..=8.0)
                    .text("Std. allein/Tag"),
            );
            ui.label("Max Größe:");
            ui.selectable_value(&mut st.profile.max_size, DogSize::Small, "Klein");
            ui.selectable_value(&mut st.profile.max_size, DogSize::Medium, "Mittel");
            ui.selectable_value(&mut st.profile.max_size, DogSize::Large, "Groß");
            ui.selectable_value(&mut st.profile.max_size, DogSize::Giant, "Riese");
        });
    });

    ui.add_space(6.0);
    let results = match_breeds(&st.profile);
    ScrollArea::vertical().show(ui, |ui| {
        for r in results.iter().take(12) {
            let score_color = if r.score >= 80 {
                pal.good
            } else if r.score >= 60 {
                pal.warn
            } else {
                pal.danger
            };
            card_frame(ui, dark).show(ui, |ui| {
                ui.horizontal(|ui| {
                    pill_badge(
                        ui,
                        &format!("{}% Match", r.score),
                        score_color,
                        Color32::WHITE,
                    );
                    ui.label(
                        RichText::new(&r.breed.name_de)
                            .strong()
                            .size(16.0)
                            .color(pal.text_primary),
                    );
                    ui.label(
                        RichText::new(format!(
                            "({}, {}-{} kg, ~{} EUR/Monat)",
                            r.breed.size.label_de(),
                            r.breed.weight_kg_min,
                            r.breed.weight_kg_max,
                            r.breed.monthly_cost_eur
                        ))
                        .color(pal.text_muted),
                    );
                });
                ui.add_space(3.0);
                ui.label(&r.breed.temperament);
                for p in &r.pros {
                    ui.colored_label(pal.good, format!("+ {p}"));
                }
                for w in &r.warnings {
                    ui.colored_label(pal.warn, format!("! {w}"));
                }
            });
            ui.add_space(4.0);
        }
    });
}

pub fn ui_encyclopedia(ui: &mut Ui, st: &mut AdvisorState) {
    ui.heading("Hundeenzyklopädie (30 Rassen + Rasse-Vergleich)");
    ui.horizontal(|ui| {
        ui.label("Suche:");
        ui.text_edit_singleline(&mut st.breed_query);
        ui.selectable_value(&mut st.compare_mode, false, "Detailansicht");
        ui.selectable_value(&mut st.compare_mode, true, "Rasse-Vergleich (A vs. B)");
    });

    let breeds = all_breeds();
    if st.compare_mode {
        ui.separator();
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Rasse A")
                .selected_text(
                    breeds
                        .iter()
                        .find(|b| b.id == st.compare_a)
                        .map_or("Wählen", |b| b.name_de.as_str()),
                )
                .show_ui(ui, |ui| {
                    for b in &breeds {
                        ui.selectable_value(&mut st.compare_a, b.id.clone(), &b.name_de);
                    }
                });
            egui::ComboBox::from_label("Rasse B")
                .selected_text(
                    breeds
                        .iter()
                        .find(|b| b.id == st.compare_b)
                        .map_or("Wählen", |b| b.name_de.as_str()),
                )
                .show_ui(ui, |ui| {
                    for b in &breeds {
                        ui.selectable_value(&mut st.compare_b, b.id.clone(), &b.name_de);
                    }
                });
        });
        if let (Some(a), Some(b)) = (
            breeds.iter().find(|x| x.id == st.compare_a),
            breeds.iter().find(|x| x.id == st.compare_b),
        ) {
            ui.add_space(6.0);
            egui::Grid::new("cmp_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label(RichText::new("Merkmal").strong());
                    ui.label(RichText::new(&a.name_de).strong());
                    ui.label(RichText::new(&b.name_de).strong());
                    ui.end_row();

                    let rows: [(&str, String, String); 10] = [
                        ("Größe", a.size.label_de().into(), b.size.label_de().into()),
                        (
                            "Gewicht",
                            format!("{}-{} kg", a.weight_kg_min, a.weight_kg_max),
                            format!("{}-{} kg", b.weight_kg_min, b.weight_kg_max),
                        ),
                        (
                            "Auslauf/Tag",
                            format!("{} Min", a.exercise_minutes_day),
                            format!("{} Min", b.exercise_minutes_day),
                        ),
                        (
                            "Kosten/Monat",
                            format!("~{} EUR", a.monthly_cost_eur),
                            format!("~{} EUR", b.monthly_cost_eur),
                        ),
                        (
                            "Energie",
                            format!("{}/5", a.energy),
                            format!("{}/5", b.energy),
                        ),
                        (
                            "Anfänger-Schwierigkeit",
                            format!("{}/5", a.beginner_difficulty),
                            format!("{}/5", b.beginner_difficulty),
                        ),
                        (
                            "Kinderfreundlich",
                            format!("{}/5", a.kid_friendly),
                            format!("{}/5", b.kid_friendly),
                        ),
                        (
                            "Katzenverträglich",
                            format!("{}/5", a.cat_friendly),
                            format!("{}/5", b.cat_friendly),
                        ),
                        (
                            "Wohnungs-Eignung",
                            format!("{}/5", a.apartment_fit),
                            format!("{}/5", b.apartment_fit),
                        ),
                        (
                            "Bellneigung",
                            format!("{}/5", a.barking),
                            format!("{}/5", b.barking),
                        ),
                    ];
                    for (lbl, va, vb) in rows {
                        ui.label(lbl);
                        ui.label(va);
                        ui.label(vb);
                        ui.end_row();
                    }
                });
        }
        return;
    }
    let q = st.breed_query.trim().to_lowercase();
    let filtered: Vec<&Breed> = breeds
        .iter()
        .filter(|b| {
            q.is_empty()
                || b.name_de.to_lowercase().contains(&q)
                || b.temperament.to_lowercase().contains(&q)
                || b.ideal_for.to_lowercase().contains(&q)
        })
        .collect();

    ui.columns(2, |cols| {
        ScrollArea::vertical()
            .id_salt("enc_list")
            .show(&mut cols[0], |ui| {
                for b in &filtered {
                    let sel = st.selected_breed_id == b.id;
                    if ui
                        .selectable_label(sel, format!("{} ({})", b.name_de, b.size.label_de()))
                        .clicked()
                    {
                        st.selected_breed_id = b.id.clone();
                    }
                }
            });

        ScrollArea::vertical()
            .id_salt("enc_detail")
            .show(&mut cols[1], |ui| {
                let dark = ui.visuals().dark_mode;
                let pal = palette(dark);
                if let Some(b) = breeds.iter().find(|x| x.id == st.selected_breed_id) {
                    card_frame(ui, dark).show(ui, |ui| {
                        ui.heading(&b.name_de);
                        ui.label(
                            RichText::new(format!(
                                "{} | Herkunft: {} | Lebenserwartung: {} Jahre",
                                b.fci_group, b.origin, b.lifespan_years
                            ))
                            .color(pal.text_muted),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            draw_breed_portrait(ui, dark, &b.id, true);
                            draw_breed_portrait(ui, dark, &b.id, false);
                        });
                        ui.add_space(6.0);
                        ui.horizontal_wrapped(|ui| {
                            pill_badge(
                                ui,
                                &format!("{}-{} kg", b.weight_kg_min, b.weight_kg_max),
                                pal.accent_soft,
                                pal.accent,
                            );
                            pill_badge(
                                ui,
                                &format!("{} Min Auslauf/Tag", b.exercise_minutes_day),
                                pal.bg_card_alt,
                                pal.text_primary,
                            );
                            pill_badge(
                                ui,
                                &format!("~{} EUR/Monat", b.monthly_cost_eur),
                                pal.bg_card_alt,
                                pal.text_primary,
                            );
                        });
                        ui.separator();
                        stat_bar(ui, dark, "Energie", b.energy, 5, pal.accent);
                        stat_bar(
                            ui,
                            dark,
                            "Anfänger-Anspruch",
                            b.beginner_difficulty,
                            5,
                            pal.warn,
                        );
                        stat_bar(ui, dark, "Kinderfreundlich", b.kid_friendly, 5, pal.good);
                        stat_bar(ui, dark, "Katzenverträglich", b.cat_friendly, 5, pal.good);
                        stat_bar(ui, dark, "Wohnungs-Eignung", b.apartment_fit, 5, pal.accent);
                        stat_bar(ui, dark, "Alm/Land-Eignung", b.rural_fit, 5, pal.good);
                        stat_bar(ui, dark, "Bellneigung", b.barking, 5, pal.warn);
                        stat_bar(ui, dark, "Haaren", b.shedding, 5, pal.warn);
                        ui.separator();
                        ui.label(RichText::new("Wesen:").strong());
                        ui.label(&b.temperament);
                        ui.label(RichText::new("Ideal für:").strong());
                        ui.label(&b.ideal_for);
                        ui.label(RichText::new("Gesundheit & Vorsorge:").strong());
                        ui.label(&b.health_notes);
                        ui.colored_label(pal.warn, format!("Achtung: {}", b.watch_out));
                    });
                }
            });
    });
}

pub fn ui_breeders_and_rescue(ui: &mut Ui, st: &mut AdvisorState) {
    ui.heading("Züchterübersicht & Rettungshunde (Woher nehmen?)");
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut st.show_rescue_tab,
            false,
            "Seriöse Züchter & Red Flags",
        );
        ui.selectable_value(
            &mut st.show_rescue_tab,
            true,
            "Rettungshunde, Tierheime & Auslandstierschutz",
        );
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        if !st.show_rescue_tab {
            ui.label(
                RichText::new("Seriöse Zuchtverbände (DACH)")
                    .strong()
                    .size(15.0),
            );
            for br in breeders() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(&br.name).strong());
                    ui.label(format!(
                        "Region: {} | Verband: {}",
                        br.region, br.association
                    ));
                    ui.label(format!("Rassen: {}", br.breeds));
                    ui.label(format!("Gesundheitschecks: {}", br.health_checks));
                    ui.label(RichText::new(&br.website).weak());
                    ui.label(&br.notes);
                });
            }
            ui.add_space(8.0);
            ui.label(
                RichText::new("WARNUNG: Woran du unseriöse Welpenhändler erkennst (Red Flags)")
                    .strong()
                    .color(Color32::from_rgb(220, 80, 80)),
            );
            for rf in breeder_red_flags() {
                ui.label(format!("• {rf}"));
            }
        } else {
            ui.label(
                RichText::new("Tierheime & Rettungshunde-Organisationen")
                    .strong()
                    .size(15.0),
            );
            for org in rescue_orgs() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(&org.name).strong());
                    ui.label(format!("Region: {} | Fokus: {}", org.region, org.focus));
                    ui.label(RichText::new(&org.website).weak());
                    ui.label(&org.notes);
                });
            }
            ui.add_space(8.0);
            ui.label(
                RichText::new("Checkliste Auslandstierschutz & Mittelmeerkrankheiten")
                    .strong()
                    .color(Color32::from_rgb(220, 160, 60)),
            );
            for item in rescue_foreign_checklist() {
                ui.label(format!("• {item}"));
            }
        }
    });
}
