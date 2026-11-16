use crate::chat_dock::ChatDockState;
use crate::settings_view::{ui_settings, SettingsState};
use crate::theme::{apply_theme, draw_paw_emblem, palette, pill_badge};
use crate::views_advisor::{ui_breeders_and_rescue, ui_encyclopedia, ui_matcher, AdvisorState};
use crate::views_care::{
    ui_legal_and_vaccines, ui_medical_and_insurance, ui_senior_dogs, ui_training_and_trainers,
    ui_travel, CareState,
};
use crate::views_life::{
    ui_calendar, ui_diary, ui_extra_tools, ui_photo_tool, ExtraToolTab, LifeState,
};
use egui::{Key, Modifiers, RichText, ScrollArea};
use puppycraft_core::skills::skills_list;
use puppycraft_core::store::PuppyStore;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Matcher,
    Encyclopedia,
    BreedersAndRescue,
    TrainingAndTrainers,
    SeniorDogs,
    MedicalAndInsurance,
    LegalAndVaccines,
    Travel,
    Diary,
    Calendar,
    PhotoTool,
    ExtraTools,
    Settings,
}

impl ActiveView {
    pub fn label(self) -> &'static str {
        match self {
            Self::Matcher => "Berater & Match",
            Self::Encyclopedia => "Enzyklopädie",
            Self::BreedersAndRescue => "Züchter & Rettung",
            Self::TrainingAndTrainers => "Erziehung & Trainer",
            Self::SeniorDogs => "Alte Hunde (Senioren)",
            Self::MedicalAndInsurance => "Medizin, Spitäler & Vers.",
            Self::LegalAndVaccines => "Recht & Impfungen",
            Self::Travel => "Reisen mit Hund",
            Self::Diary => "Hundsi-Tagebuch",
            Self::Calendar => "Verleih-Kalender",
            Self::PhotoTool => "Hundsi-Phototool",
            Self::ExtraTools => "Mehr Tools (20+)",
            Self::Settings => "LLM & Settings",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpTab {
    QuickStart,
    Readme,
    Skills,
    Shortcuts,
}

const ZOOM_MIN: f32 = 0.65;
const ZOOM_MAX: f32 = 2.2;

pub struct PuppyCraftApp {
    pub store: Arc<PuppyStore>,
    pub active_view: ActiveView,
    pub dark_mode: bool,
    pub zoom: f32,
    pub advisor: AdvisorState,
    pub care: CareState,
    pub life: LifeState,
    pub chat: ChatDockState,
    pub settings: SettingsState,
    pub show_docs_modal: bool,
    pub help_tab: HelpTab,
    pub show_about_modal: bool,
    pub show_shortcuts_modal: bool,
    pub show_palette_modal: bool,
    pub palette_query: String,
    pub selected_skill_idx: usize,
}

impl PuppyCraftApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let store = match PuppyStore::open_default() {
            Ok(s) => Arc::new(s),
            Err(_) => {
                // Fallback in-memory if disk path is unwritable
                #[allow(clippy::unwrap_used)]
                Arc::new(PuppyStore::open_in_memory().unwrap_or_else(|_| unreachable!()))
            }
        };
        let chat = ChatDockState::from_store(&store);
        Self {
            store,
            active_view: ActiveView::Matcher,
            dark_mode: true,
            zoom: 1.0,
            advisor: AdvisorState::default(),
            care: CareState::default(),
            life: LifeState::default(),
            chat,
            settings: SettingsState::default(),
            show_docs_modal: false,
            help_tab: HelpTab::QuickStart,
            show_about_modal: false,
            show_shortcuts_modal: false,
            show_palette_modal: false,
            palette_query: String::new(),
            selected_skill_idx: 0,
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.options_mut(|o| o.zoom_with_keyboard = true);
        if (ctx.zoom_factor() - self.zoom).abs() > 1e-3 {
            ctx.set_zoom_factor(self.zoom.clamp(ZOOM_MIN, ZOOM_MAX));
        }
        let wheel = ctx.input(|i| i.zoom_delta());
        if (wheel - 1.0).abs() > f32::EPSILON {
            self.zoom = (ctx.zoom_factor() * wheel).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| {
            i.consume_key(Modifiers::CTRL, Key::Equals) || i.consume_key(Modifiers::CTRL, Key::Plus)
        }) {
            self.zoom = (self.zoom * 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Minus)) {
            self.zoom = (self.zoom / 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
            ctx.set_zoom_factor(self.zoom);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Num0)) {
            self.zoom = 1.0;
            ctx.set_zoom_factor(1.0);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F1)) {
            self.show_docs_modal = !self.show_docs_modal;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::J)) {
            self.chat.open = !self.chat.open;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::K)) {
            self.show_palette_modal = !self.show_palette_modal;
        }
    }

    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Neu: Tagebuch-Eintrag").clicked() {
                    self.active_view = ActiveView::Diary;
                    ui.close();
                }
                if ui.button("Neu: Verleih-Termin").clicked() {
                    self.active_view = ActiveView::Calendar;
                    ui.close();
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("LLM & Chat-Einstellungen...").clicked() {
                    self.active_view = ActiveView::Settings;
                    ui.close();
                }
            });

            ui.menu_button("View", |ui| {
                if ui
                    .button(if self.dark_mode {
                        "Theme: Light Mode"
                    } else {
                        "Theme: Dark Mode"
                    })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                    ui.close();
                }
                ui.separator();
                if ui.button("Zoom In (Ctrl+= / Ctrl+Wheel)").clicked() {
                    self.zoom = (self.zoom * 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
                    ui.close();
                }
                if ui.button("Zoom Out (Ctrl+- / Ctrl+Wheel)").clicked() {
                    self.zoom = (self.zoom / 1.1).clamp(ZOOM_MIN, ZOOM_MAX);
                    ui.close();
                }
                if ui.button("Reset Zoom 100% (Ctrl+0)").clicked() {
                    self.zoom = 1.0;
                    ui.close();
                }
            });

            ui.menu_button("Hund", |ui| {
                if ui.button("Berater & Match").clicked() {
                    self.active_view = ActiveView::Matcher;
                    ui.close();
                }
                if ui.button("Enzyklopädie (30 Rassen)").clicked() {
                    self.active_view = ActiveView::Encyclopedia;
                    ui.close();
                }
                if ui.button("Züchter & Rettungshunde").clicked() {
                    self.active_view = ActiveView::BreedersAndRescue;
                    ui.close();
                }
                if ui.button("Welpenerziehung & Trainer").clicked() {
                    self.active_view = ActiveView::TrainingAndTrainers;
                    ui.close();
                }
                if ui.button("Recht & Impfungen").clicked() {
                    self.active_view = ActiveView::LegalAndVaccines;
                    ui.close();
                }
                if ui.button("Reisen (Wien / Tokio)").clicked() {
                    self.active_view = ActiveView::Travel;
                    ui.close();
                }
            });

            ui.menu_button("Werkzeuge", |ui| {
                if ui.button("Gift-Lexikon & Schoko-Rechner").clicked() {
                    self.active_view = ActiveView::ExtraTools;
                    self.life.extra_tab = ExtraToolTab::ToxicAndChoco;
                    ui.close();
                }
                if ui.button("Futter- & Kostenrechner").clicked() {
                    self.active_view = ActiveView::ExtraTools;
                    self.life.extra_tab = ExtraToolTab::FoodCalc;
                    ui.close();
                }
                if ui.button("Hundsi-Phototool").clicked() {
                    self.active_view = ActiveView::PhotoTool;
                    ui.close();
                }
            });

            ui.menu_button("Window", |ui| {
                if ui.button("Chat (Ctrl+J)").clicked() {
                    self.chat.open = !self.chat.open;
                    ui.close();
                }
                if ui.button("Reset Layout").clicked() {
                    self.chat.open = true;
                    self.chat.maximized = false;
                    ui.close();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Handbuch & Kurzanleitung (F1)...").clicked() {
                    self.help_tab = HelpTab::QuickStart;
                    self.show_docs_modal = true;
                    ui.close();
                }
                if ui.button("Project Files & Docs (Skills)...").clicked() {
                    self.help_tab = HelpTab::Skills;
                    self.show_docs_modal = true;
                    ui.close();
                }
                if ui.button("Command Palette... (Ctrl+K)").clicked() {
                    self.show_palette_modal = true;
                    ui.close();
                }
                if ui.button("Keyboard Shortcuts...").clicked() {
                    self.show_shortcuts_modal = true;
                    ui.close();
                }
                ui.separator();
                if ui.button("About PuppyCraft").clicked() {
                    self.show_about_modal = true;
                    ui.close();
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .small_button(if self.dark_mode { "Light" } else { "Dark" })
                    .clicked()
                {
                    self.dark_mode = !self.dark_mode;
                }
                if ui
                    .small_button(format!("{:.0}%", self.zoom * 100.0))
                    .on_hover_text("Ctrl+Wheel / Ctrl+= / Ctrl+- / Klick = Reset 100%")
                    .clicked()
                {
                    self.zoom = 1.0;
                }
                if ui
                    .small_button(if self.chat.open {
                        "Chat: AN"
                    } else {
                        "Chat: AUS"
                    })
                    .clicked()
                {
                    self.chat.open = !self.chat.open;
                }
            });
        });
    }
}

impl eframe::App for PuppyCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        apply_theme(&ctx, self.dark_mode);
        let pal = palette(self.dark_mode);
        self.handle_shortcuts(&ctx);
        self.chat.poll(&self.store, self.active_view.label());
        self.settings.poll(&mut self.chat, &self.store);
        if self.chat.open_settings_requested {
            self.chat.open_settings_requested = false;
            self.chat.maximized = false;
            self.active_view = ActiveView::Settings;
        }
        if self.chat.busy || self.settings.scanning || self.settings.testing {
            ctx.request_repaint();
        }

        egui::Panel::top("top_menu").show(ui, |ui| {
            self.render_menu_bar(ui);
        });

        if self.chat.open && self.chat.maximized {
            egui::CentralPanel::default().show(ui, |ui| {
                self.chat.ui(ui, &self.store, self.active_view.label());
            });
            return;
        }

        egui::Panel::left("nav_rail")
            .default_size(224.0)
            .show(ui, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    draw_paw_emblem(ui, 30.0, pal.accent);
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("PuppyCraft")
                                .strong()
                                .size(19.0)
                                .color(pal.text_primary),
                        );
                        ui.label(
                            RichText::new("Hundeberatung & Alltag")
                                .size(11.5)
                                .color(pal.text_muted),
                        );
                    });
                });
                ui.add_space(4.0);
                if let Ok(dogs) = self.store.list_dogs() {
                    if let Some(d) = dogs.first() {
                        pill_badge(
                            ui,
                            &format!("{} • {:.1} kg", d.name, d.weight_kg),
                            pal.accent_soft,
                            pal.accent,
                        );
                    }
                }
                ui.add_space(6.0);
                ui.separator();
                let views = [
                    ActiveView::Matcher,
                    ActiveView::Encyclopedia,
                    ActiveView::BreedersAndRescue,
                    ActiveView::TrainingAndTrainers,
                    ActiveView::SeniorDogs,
                    ActiveView::MedicalAndInsurance,
                    ActiveView::LegalAndVaccines,
                    ActiveView::Travel,
                    ActiveView::Diary,
                    ActiveView::Calendar,
                    ActiveView::PhotoTool,
                    ActiveView::ExtraTools,
                    ActiveView::Settings,
                ];
                for v in views {
                    let active = self.active_view == v;
                    let resp = ui.add_sized(
                        [ui.available_width(), 30.0],
                        egui::Button::selectable(active, RichText::new(v.label()).size(13.5)),
                    );
                    if resp.clicked() {
                        self.active_view = v;
                    }
                }
            });

        if self.chat.open {
            egui::Panel::right("chat_dock")
                .default_size(360.0)
                .min_size(280.0)
                .max_size(480.0)
                .resizable(true)
                .show(ui, |ui| {
                    self.chat.ui(ui, &self.store, self.active_view.label());
                });
        }

        egui::CentralPanel::default().show(ui, |ui| match self.active_view {
            ActiveView::Matcher => ui_matcher(ui, &mut self.advisor),
            ActiveView::Encyclopedia => ui_encyclopedia(ui, &mut self.advisor),
            ActiveView::BreedersAndRescue => ui_breeders_and_rescue(ui, &mut self.advisor),
            ActiveView::TrainingAndTrainers => ui_training_and_trainers(ui, &mut self.care),
            ActiveView::SeniorDogs => ui_senior_dogs(ui),
            ActiveView::MedicalAndInsurance => ui_medical_and_insurance(ui, &mut self.care),
            ActiveView::LegalAndVaccines => ui_legal_and_vaccines(ui, &mut self.care, &self.store),
            ActiveView::Travel => ui_travel(ui),
            ActiveView::Diary => ui_diary(ui, &mut self.life, &self.store),
            ActiveView::Calendar => ui_calendar(ui, &mut self.life, &self.store),
            ActiveView::PhotoTool => ui_photo_tool(ui, &mut self.life, &self.store),
            ActiveView::ExtraTools => ui_extra_tools(ui, &mut self.life, &self.store),
            ActiveView::Settings => {
                ui_settings(ui, &mut self.settings, &mut self.chat, &self.store)
            }
        });

        if self.show_docs_modal {
            egui::Window::new("PuppyCraft Handbuch, Docs & Bundled Skills")
                .open(&mut self.show_docs_modal)
                .default_width(680.0)
                .show(&ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.help_tab, HelpTab::QuickStart, "Kurzanleitung");
                        ui.selectable_value(&mut self.help_tab, HelpTab::Readme, "README");
                        ui.selectable_value(&mut self.help_tab, HelpTab::Skills, "Bundled Skills");
                        ui.selectable_value(&mut self.help_tab, HelpTab::Shortcuts, "Tastenkürzel");
                    });
                    ui.separator();
                    ScrollArea::vertical().max_height(430.0).show(ui, |ui| match self.help_tab {
                        HelpTab::QuickStart => {
                            ui.heading("Willkommen bei PuppyCraft");
                            ui.label("1. Berater & Match: Wohnort (Almöhi vs. Innenstadt), Familienstand (Single, Kinder, Katze), Erfahrung und Zeitbudget wählen -> Top-Rassen mit Welpen- & Erwachsenenbildern.");
                            ui.label("2. Enzyklopädie: 30 Rassen durchsuchen und Welpen- vs. Erwachsenen-Porträt direkt vergleichen.");
                            ui.label("3. Züchter, Rettung, Erziehung & Senioren: FCI/ÖKV-Checkliste, Tierschutz, Trainingspläne und Pflege im Alter.");
                            ui.label("4. Medizin, Spitäler & Versicherung: Rasse-Krankheiten (HD, Schliafhansl, Allergien), Notfall-Spitäler (Vetmeduni Wien, Hofheim etc.) und Haftpflicht-/OP-Versicherungen.");
                            ui.label("5. Recht, Impfungen & Reisen: EU-Heimtierausweis, Chippflicht, Wien vs. Tokio Regeln.");
                            ui.label("6. Hundsi-Tagebuch, Verleih-Kalender & Phototool: Gewicht, Impfungen, Sitter-Termine und Fotopresets.");
                            ui.label("7. AI Berater-Chat (Ctrl+J): Fragen zu Rassen, Erziehung oder Notfällen (z.B. Schoko-Rechner) direkt mit den gebündelten Skills stellen.");
                        }
                        HelpTab::Readme => {
                            ui.monospace(include_str!("../../../README.md"));
                        }
                        HelpTab::Skills => {
                            let skills = skills_list();
                            ui.horizontal(|ui| {
                                for (idx, s) in skills.iter().enumerate() {
                                    if ui
                                        .selectable_label(self.selected_skill_idx == idx, s.name)
                                        .clicked()
                                    {
                                        self.selected_skill_idx = idx;
                                    }
                                }
                            });
                            ui.separator();
                            if let Some(s) = skills.get(self.selected_skill_idx) {
                                ui.monospace(s.skill_md);
                            }
                        }
                        HelpTab::Shortcuts => {
                            ui.label("F1 — Handbuch & Kurzanleitung öffnen/schließen");
                            ui.label("Ctrl+J — Hund-Berater Chat Dock ein-/ausblenden");
                            ui.label("Ctrl+K — Command Palette");
                            ui.label("Ctrl+Mausrad / Ctrl+= / Ctrl+- — UI Zoom rein/raus");
                            ui.label("Ctrl+0 — Zoom auf 100% zurücksetzen");
                        }
                    });
                });
        }

        if self.show_shortcuts_modal {
            egui::Window::new("Keyboard Shortcuts")
                .open(&mut self.show_shortcuts_modal)
                .show(&ctx, |ui| {
                    ui.label("F1 — Handbuch & Kurzanleitung");
                    ui.label("Ctrl+J — Hund-Berater Chat Dock");
                    ui.label("Ctrl+K — Command Palette");
                    ui.label("Ctrl+Wheel / Ctrl+= / Ctrl+- — Zoom In / Out");
                    ui.label("Ctrl+0 — Reset Zoom (100%)");
                });
        }

        if self.show_about_modal {
            egui::Window::new("About PuppyCraft")
                .open(&mut self.show_about_modal)
                .show(&ctx, |ui| {
                    ui.heading(format!("PuppyCraft v{}", env!("CARGO_PKG_VERSION")));
                    ui.label("Pure-Rust ArtCraft Workstation für Hundeberatung, Enzyklopädie, Erziehung, Recht, Reisen, Hundsi-Tagebuch, Verleih-Kalender & Phototool.");
                    ui.label("License: MIT OR Apache-2.0");
                });
        }

        if self.show_palette_modal {
            let mut open = true;
            let mut close_palette = false;
            egui::Window::new("Command Palette")
                .open(&mut open)
                .show(&ctx, |ui| {
                    ui.text_edit_singleline(&mut self.palette_query);
                    let q = self.palette_query.to_lowercase();
                    let views = [
                        ActiveView::Matcher,
                        ActiveView::Encyclopedia,
                        ActiveView::BreedersAndRescue,
                        ActiveView::TrainingAndTrainers,
                        ActiveView::SeniorDogs,
                        ActiveView::MedicalAndInsurance,
                        ActiveView::LegalAndVaccines,
                        ActiveView::Travel,
                        ActiveView::Diary,
                        ActiveView::Calendar,
                        ActiveView::PhotoTool,
                        ActiveView::ExtraTools,
                    ];
                    for v in views {
                        if (q.is_empty() || v.label().to_lowercase().contains(&q))
                            && ui.button(v.label()).clicked()
                        {
                            self.active_view = v;
                            close_palette = true;
                        }
                    }
                });
            if close_palette || !open {
                self.show_palette_modal = false;
            }
        }
    }
}
