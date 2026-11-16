use crate::theme::{card_frame, hero_header, palette, pill_badge};
use egui::{Color32, RichText, ScrollArea, Ui};
use puppycraft_core::knowledge::{
    insurance_providers, legal_rules, medical_problems, senior_dog_sections, trainers,
    training_guides, travel_destinations, travel_packing_list, vaccine_schedule_template,
    vet_clinics,
};
use puppycraft_core::store::PuppyStore;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MedicalTab {
    Problems,
    ClinicsAndHospitals,
    Insurance,
}

pub struct CareState {
    pub show_trainers_tab: bool,
    pub show_vaccines_tab: bool,
    pub medical_tab: MedicalTab,
    pub vax_dog: String,
    pub vax_name: String,
    pub vax_given: String,
    pub vax_due: String,
    pub vax_clinic: String,
    pub vax_notes: String,
    pub vax_msg: String,
}

impl Default for CareState {
    fn default() -> Self {
        Self {
            show_trainers_tab: false,
            show_vaccines_tab: false,
            medical_tab: MedicalTab::Problems,
            vax_dog: "Beppo".into(),
            vax_name: "Tollwut Auffrischung".into(),
            vax_given: "2026-11-15".into(),
            vax_due: "2029-11-15".into(),
            vax_clinic: "Tierklinik Wien".into(),
            vax_notes: String::new(),
            vax_msg: String::new(),
        }
    }
}

pub fn ui_training_and_trainers(ui: &mut Ui, st: &mut CareState) {
    ui.heading("Welpenerziehung, Hundetrainer & Abrichteplätze");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut st.show_trainers_tab, false, "Welpenerziehung & Phasen");
        ui.selectable_value(
            &mut st.show_trainers_tab,
            true,
            "Hundetrainer & Abrichteplätze",
        );
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        if !st.show_trainers_tab {
            for g in training_guides() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(format!("{} — {}", g.phase, g.title)).strong().size(15.0));
                    ui.label(&g.summary);
                    ui.add_space(4.0);
                    for s in &g.steps {
                        ui.label(format!("• {s}"));
                    }
                    ui.colored_label(
                        Color32::from_rgb(220, 130, 60),
                        format!("Häufige Fehler: {}", g.common_mistakes),
                    );
                });
            }
        } else {
            ui.label(
                RichText::new("Seriöse Verbände, Abrichteplätze & Qualitätskriterien")
                    .strong()
                    .size(15.0),
            );
            for t in trainers() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new(&t.name).strong());
                    ui.label(format!("Orte: {} | Verband: {}", t.city, t.association));
                    ui.label(format!("Schwerpunkte: {}", t.specialties));
                    ui.label(format!("Methodik: {}", t.methodology));
                    ui.label(RichText::new(&t.notes).weak());
                });
            }
            ui.add_space(6.0);
            ui.colored_label(
                Color32::from_rgb(220, 80, 80),
                "Finger weg von Trainern mit Stachelhalsband, Leinenruck, Alphawurf oder Wasserflasche!",
            );
        }
    });
}

pub fn ui_legal_and_vaccines(ui: &mut Ui, st: &mut CareState, store: &Arc<PuppyStore>) {
    ui.heading("Rechtliches (AT/DE/CH) & Impfungen");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut st.show_vaccines_tab, false, "Rechtliches & Gesetze");
        ui.selectable_value(
            &mut st.show_vaccines_tab,
            true,
            "Impfungen & Persönlicher Impfpass",
        );
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| {
        if !st.show_vaccines_tab {
            for r in legal_rules() {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("{} — {}", r.region, r.topic))
                            .strong()
                            .size(15.0),
                    );
                    ui.label(RichText::new(&r.summary).italics());
                    ui.add_space(3.0);
                    ui.label(&r.details);
                });
            }
        } else {
            ui.label(RichText::new("Standard-Impfschema (WSAVA / StIKo Vet)").strong());
            for (name, desc, cat) in vaccine_schedule_template() {
                ui.label(format!("• [{cat}] {name}: {desc}"));
            }
            ui.separator();

            ui.label(RichText::new("Neue Impfung eintragen").strong());
            egui::Grid::new("vax_form").num_columns(2).show(ui, |ui| {
                ui.label("Hund:");
                ui.text_edit_singleline(&mut st.vax_dog);
                ui.end_row();
                ui.label("Impfung:");
                ui.text_edit_singleline(&mut st.vax_name);
                ui.end_row();
                ui.label("Datum (YYYY-MM-DD):");
                ui.text_edit_singleline(&mut st.vax_given);
                ui.end_row();
                ui.label("Nächste fällig:");
                ui.text_edit_singleline(&mut st.vax_due);
                ui.end_row();
                ui.label("Tierarzt / Klinik:");
                ui.text_edit_singleline(&mut st.vax_clinic);
                ui.end_row();
                ui.label("Notiz:");
                ui.text_edit_singleline(&mut st.vax_notes);
                ui.end_row();
            });
            if ui.button("Impfung speichern").clicked() {
                match store.add_vaccination(
                    &st.vax_dog,
                    &st.vax_name,
                    &st.vax_given,
                    &st.vax_due,
                    &st.vax_clinic,
                    &st.vax_notes,
                ) {
                    Ok(id) => st.vax_msg = format!("Gespeichert (ID {id})"),
                    Err(e) => st.vax_msg = format!("Fehler: {e}"),
                }
            }
            if !st.vax_msg.is_empty() {
                ui.label(&st.vax_msg);
            }

            ui.separator();
            ui.label(RichText::new("Gespeicherte Impfungen").strong());
            if let Ok(list) = store.list_vaccinations() {
                for v in list {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(
                            RichText::new(format!(
                                "{} — {} ({})",
                                v.dog_name, v.vaccine_name, v.vet_clinic
                            ))
                            .strong(),
                        );
                        ui.label(format!(
                            "Geimpft: {} | Nächste fällig: {}",
                            v.date_given, v.next_due
                        ));
                        if !v.notes.is_empty() {
                            ui.label(&v.notes);
                        }
                    });
                }
            }
        }
    });
}

pub fn ui_travel(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Reisen mit Hund (Wien vs. Tokio & Europa)",
        "Wo kommt dein Hund überall rein, und wo ist es extrem streng?",
    );

    ScrollArea::vertical().show(ui, |ui| {
        for d in travel_destinations() {
            let color = if d.dog_friendliness_score >= 75 {
                pal.good
            } else if d.dog_friendliness_score >= 50 {
                pal.warn
            } else {
                pal.danger
            };
            card_frame(ui, dark).show(ui, |ui| {
                ui.horizontal(|ui| {
                    pill_badge(
                        ui,
                        &format!("{}/100", d.dog_friendliness_score),
                        color,
                        Color32::WHITE,
                    );
                    ui.label(RichText::new(&d.city_or_country).strong().size(16.0));
                    ui.label(RichText::new(format!("— {}", d.rating_label)).color(color));
                });
                ui.add_space(4.0);
                ui.label(RichText::new("Öffis & Transport:").strong());
                ui.label(&d.transit_rules);
                ui.label(RichText::new("Restaurants & Geschäfte:").strong());
                ui.label(&d.restaurants_shops);
                ui.label(RichText::new("Einreisebestimmungen:").strong());
                ui.label(&d.entry_requirements);
                ui.label(RichText::new("Lokale Tipps:").strong());
                ui.label(&d.local_tips);
            });
            ui.add_space(5.0);
        }

        ui.add_space(6.0);
        card_frame(ui, dark).show(ui, |ui| {
            ui.label(
                RichText::new("Reise-Packliste für den Hund")
                    .strong()
                    .size(15.0),
            );
            for item in travel_packing_list() {
                ui.label(format!("• {item}"));
            }
        });
    });
}

pub fn ui_senior_dogs(ui: &mut Ui) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Abteilung Alte Hunde (Senioren-Betreuung)",
        "Arthrose, Hunde-Demenz (CCD), Herz/Nieren-Check, Ernährung & würdevoller Abschied",
    );

    ScrollArea::vertical().show(ui, |ui| {
        for sec in senior_dog_sections() {
            card_frame(ui, dark).show(ui, |ui| {
                ui.label(
                    RichText::new(&sec.title)
                        .strong()
                        .size(15.5)
                        .color(pal.accent),
                );
                ui.label(&sec.summary);
                ui.add_space(4.0);
                for t in &sec.tips {
                    ui.label(format!("• {t}"));
                }
            });
            ui.add_space(5.0);
        }
    });
}

pub fn ui_medical_and_insurance(ui: &mut Ui, st: &mut CareState) {
    let dark = ui.visuals().dark_mode;
    let pal = palette(dark);
    hero_header(
        ui,
        dark,
        "Medizin, Tierspitäler & Hundeversicherung",
        "Typische Probleme (HD, Schliafhansel, Allergien), Praxen/Tierspitäler & Haftpflicht/Gesundheit",
    );

    ui.horizontal_wrapped(|ui| {
        ui.selectable_value(
            &mut st.medical_tab,
            MedicalTab::Problems,
            "Typische Probleme (HD, Schliafhansel, Allergien)",
        );
        ui.selectable_value(
            &mut st.medical_tab,
            MedicalTab::ClinicsAndHospitals,
            "Praxen, Überweisung & Tierspitäler",
        );
        ui.selectable_value(
            &mut st.medical_tab,
            MedicalTab::Insurance,
            "Haftpflicht & Krankenversicherung",
        );
    });
    ui.separator();

    ScrollArea::vertical().show(ui, |ui| match st.medical_tab {
        MedicalTab::Problems => {
            for m in medical_problems() {
                let urg_col = if m.urgency.contains("AKUT") || m.urgency.contains("HOCH") {
                    pal.danger
                } else {
                    pal.warn
                };
                card_frame(ui, dark).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(&m.name).strong().size(15.5));
                        pill_badge(ui, &m.urgency, urg_col, Color32::WHITE);
                    });
                    ui.label(
                        RichText::new(format!("Häufig betroffen: {}", m.affected_breeds))
                            .color(pal.text_muted),
                    );
                    ui.label(RichText::new("Symptome:").strong());
                    ui.label(&m.symptoms);
                    ui.label(RichText::new("Vorbeugung & Handeln:").strong());
                    ui.label(&m.prevention_and_action);
                });
                ui.add_space(5.0);
            }
        }
        MedicalTab::ClinicsAndHospitals => {
            for c in vet_clinics() {
                card_frame(ui, dark).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        pill_badge(
                            ui,
                            &c.tier_type,
                            if c.emergency_24h {
                                pal.danger
                            } else {
                                pal.accent_soft
                            },
                            if c.emergency_24h {
                                Color32::WHITE
                            } else {
                                pal.accent
                            },
                        );
                        ui.label(RichText::new(&c.name).strong().size(15.0));
                    });
                    ui.label(
                        RichText::new(format!(
                            "Region: {} | Schwerpunkte: {}",
                            c.region, c.specialties
                        ))
                        .color(pal.text_muted),
                    );
                    ui.label(&c.when_to_go);
                });
                ui.add_space(5.0);
            }
        }
        MedicalTab::Insurance => {
            for ins in insurance_providers() {
                card_frame(ui, dark).show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(&ins.name).strong().size(15.0));
                        pill_badge(ui, &ins.monthly_eur, pal.accent_soft, pal.accent);
                    });
                    ui.label(RichText::new(&ins.category).color(pal.good));
                    ui.label(format!("Deckung: {}", ins.coverage));
                    ui.label(format!("Wartezeit: {}", ins.waiting_period));
                    ui.label(RichText::new(&ins.notes).color(pal.text_muted));
                });
                ui.add_space(5.0);
            }
        }
    });
}
