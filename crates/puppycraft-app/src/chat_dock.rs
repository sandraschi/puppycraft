use crate::theme::palette;
use egui::{Color32, CornerRadius, Frame, Margin, RichText, ScrollArea, Stroke, Ui};
use puppycraft_core::command::{dispatch_tool, is_read_only_tool};
use puppycraft_core::llm::{
    append_chat_transcript, build_preprompt, call_llm_turn, load_llm_config, ChatMessage,
    LlmConfig, LlmTurnOutput, PendingToolCall,
};
use puppycraft_core::store::PuppyStore;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;

pub enum ChatWorkerMsg {
    TurnDone(Result<LlmTurnOutput, String>),
}

pub struct ChatDockState {
    pub open: bool,
    pub maximized: bool,
    pub cfg: LlmConfig,
    pub input: String,
    pub messages: Vec<ChatMessage>,
    pub pending_confirm: Option<PendingToolCall>,
    pub last_error: Option<String>,
    pub show_prompt_modal: bool,
    pub open_settings_requested: bool,
    pub last_preprompt: String,
    pub last_tokens: usize,
    pub busy: bool,
    pub rx: Option<Receiver<ChatWorkerMsg>>,
    pub tool_rounds: u8,
}

impl Default for ChatDockState {
    fn default() -> Self {
        Self {
            open: true,
            maximized: false,
            cfg: LlmConfig::default(),
            input: String::new(),
            messages: vec![ChatMessage {
                role: "assistant".into(),
                content: "Hallo! Ich bin der PuppyCraft-Berater (mit Hund-Skill). Frag mich z.B.:\n- 'Welcher Hund passt zu einem einsamen Oldie in Wien?'\n- 'Familie mit 3 Kids und Katze - worauf achten?'\n- 'Darf mein Hund in Tokio in die U-Bahn oder ins Restaurant?'\n- 'Mein 10 kg Hund hat 40g Zartbitterschokolade gefressen!'".into(),
                tool_calls: None,
                tool_call_id: None,
            }],
            pending_confirm: None,
            last_error: None,
            show_prompt_modal: false,
            open_settings_requested: false,
            last_preprompt: String::new(),
            last_tokens: 0,
            busy: false,
            rx: None,
            tool_rounds: 0,
        }
    }
}

impl ChatDockState {
    pub fn from_store(store: &PuppyStore) -> Self {
        Self {
            cfg: load_llm_config(store),
            ..Self::default()
        }
    }

    pub fn poll(&mut self, store: &Arc<PuppyStore>, active_view: &str) {
        let Some(rx) = self.rx.as_ref() else {
            return;
        };
        if let Ok(ChatWorkerMsg::TurnDone(res)) = rx.try_recv() {
            self.busy = false;
            self.rx = None;
            match res {
                Ok(LlmTurnOutput::AssistantText(txt)) => {
                    self.last_error = None;
                    self.tool_rounds = 0;
                    if self.cfg.save_transcripts {
                        append_chat_transcript("assistant", &txt);
                    }
                    self.messages.push(ChatMessage {
                        role: "assistant".into(),
                        content: txt,
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
                Ok(LlmTurnOutput::ToolCalls(tcs)) => {
                    if let Some(tc) = tcs.into_iter().next() {
                        if self.tool_rounds >= 6 {
                            self.last_error = Some("Maximale Tool-Runden (6) erreicht.".into());
                            self.tool_rounds = 0;
                            return;
                        }
                        if is_read_only_tool(&tc.name) {
                            self.run_tool_and_continue(store, active_view, &tc);
                        } else {
                            self.pending_confirm = Some(tc);
                        }
                    }
                }
                Err(e) => {
                    self.last_error = Some(e);
                }
            }
        }
    }

    fn run_tool_and_continue(
        &mut self,
        store: &Arc<PuppyStore>,
        active_view: &str,
        tc: &PendingToolCall,
    ) {
        self.tool_rounds += 1;
        let out = match dispatch_tool(store, &tc.name, &tc.arguments) {
            Ok(v) => v.to_string(),
            Err(e) => format!("{{\"error\": \"{e}\"}}"),
        };
        self.messages.push(ChatMessage {
            role: "tool".into(),
            content: format!("[Tool {} Ergebnis]: {}", tc.name, out),
            tool_calls: None,
            tool_call_id: Some(tc.id.clone()),
        });
        self.spawn_turn(store, active_view);
    }

    pub fn send_user_message(&mut self, store: &Arc<PuppyStore>, active_view: &str) {
        let text = self.input.trim().to_string();
        if text.is_empty() || self.busy {
            return;
        }
        self.input.clear();
        self.last_error = None;
        self.tool_rounds = 0;
        if self.cfg.save_transcripts {
            append_chat_transcript("user", &text);
        }
        self.messages.push(ChatMessage {
            role: "user".into(),
            content: text,
            tool_calls: None,
            tool_call_id: None,
        });
        self.spawn_turn(store, active_view);
    }

    fn spawn_turn(&mut self, store: &Arc<PuppyStore>, active_view: &str) {
        let prep = build_preprompt(store, active_view, &self.cfg.user_extra_prompt);
        self.last_preprompt = prep.full_text.clone();
        self.last_tokens = prep.estimated_tokens;
        let cfg = self.cfg.clone();
        let history = self.messages.clone();

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        self.busy = true;
        std::thread::spawn(move || {
            let res = call_llm_turn(&cfg, &prep.full_text, &history).map_err(|e| e.to_string());
            let _ = tx.send(ChatWorkerMsg::TurnDone(res));
        });
    }

    pub fn ui(&mut self, ui: &mut Ui, store: &Arc<PuppyStore>, active_view: &str) {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
        ui.horizontal(|ui| {
            ui.heading("Hund-Berater Chat");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button("X").clicked() {
                    self.open = false;
                }
                if ui
                    .small_button(if self.maximized { "Normal" } else { "Max" })
                    .clicked()
                {
                    self.maximized = !self.maximized;
                }
                if ui.small_button("Clear").clicked() {
                    self.messages.truncate(1);
                    self.last_error = None;
                    self.pending_confirm = None;
                }
            });
        });

        ui.horizontal_wrapped(|ui| {
            let model_txt = if self.cfg.model.trim().is_empty() {
                "Auto / Settings..."
            } else {
                self.cfg.model.as_str()
            };
            let pill = format!("{} | {}", self.cfg.provider.label(), model_txt);
            if ui.small_button(pill).clicked() {
                self.open_settings_requested = true;
            }
            ui.checkbox(&mut self.cfg.tools_enabled, "Tools");
            if ui.small_button("View prompt").clicked() {
                let prep = build_preprompt(store, active_view, &self.cfg.user_extra_prompt);
                self.last_preprompt = prep.full_text;
                self.last_tokens = prep.estimated_tokens;
                self.show_prompt_modal = true;
            }
            ui.label(
                RichText::new(format!("~{} tok / 16k", self.last_tokens))
                    .small()
                    .weak(),
            );
        });

        if !self.cfg.provider.is_local(&self.cfg.base_url) {
            ui.colored_label(
                Color32::from_rgb(210, 140, 40),
                format!(
                    "Hinweis: Chat sendet App-Daten an {}.",
                    self.cfg.provider.label()
                ),
            );
        }

        ui.separator();

        let dark = ui.visuals().dark_mode;
        let pal = palette(dark);

        ScrollArea::vertical()
            .id_salt("chat_scroll")
            .max_height(ui.available_height() - 125.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for m in &self.messages {
                    let (prefix, color, fill) = match m.role.as_str() {
                        "user" => ("Du", pal.accent, pal.accent_soft),
                        "tool" => ("Tool", pal.good, pal.bg_card_alt),
                        _ => ("PuppyCraft", pal.text_primary, pal.bg_card),
                    };
                    Frame::new()
                        .fill(fill)
                        .stroke(Stroke::new(1.0, pal.border))
                        .corner_radius(CornerRadius::same(10))
                        .inner_margin(Margin::same(9))
                        .show(ui, |ui| {
                            ui.label(RichText::new(prefix).size(11.5).strong().color(color));
                            ui.label(&m.content);
                        });
                    ui.add_space(5.0);
                }

                if let Some(tc) = self.pending_confirm.clone() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(
                            RichText::new(format!("Schreib-Aktion bestätigen: {}", tc.name))
                                .strong()
                                .color(Color32::from_rgb(220, 160, 50)),
                        );
                        ui.label(tc.arguments.to_string());
                        ui.horizontal(|ui| {
                            if ui.button("Allow once").clicked() {
                                self.pending_confirm = None;
                                self.run_tool_and_continue(store, active_view, &tc);
                            }
                            if ui.button("Deny").clicked() {
                                self.pending_confirm = None;
                                self.messages.push(ChatMessage {
                                    role: "tool".into(),
                                    content: "Vom Benutzer abgelehnt.".into(),
                                    tool_calls: None,
                                    tool_call_id: Some(tc.id),
                                });
                            }
                        });
                    });
                }

                if let Some(err) = self.last_error.clone() {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.colored_label(Color32::from_rgb(220, 80, 80), format!("Fehler: {err}"));
                        ui.horizontal(|ui| {
                            if ui.small_button("Retry").clicked() {
                                self.last_error = None;
                                self.spawn_turn(store, active_view);
                            }
                            if ui.small_button("LLM & Settings").clicked() {
                                self.open_settings_requested = true;
                            }
                        });
                    });
                }
            });

        ui.separator();
        if self.busy {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Denke nach...");
            });
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let send_clicked = ui
                .add_enabled(!self.busy, egui::Button::new("Senden"))
                .clicked();
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.input)
                    .hint_text("Frage zum Hund, Rasse, Erziehung, Tokio/Wien...")
                    .desired_width(ui.available_width()),
            );
            let enter = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (send_clicked || enter) && !self.busy {
                self.send_user_message(store, active_view);
            }
        });

        if self.show_prompt_modal {
            egui::Window::new("Chat Preprompt (Skills + Live Status)")
                .open(&mut self.show_prompt_modal)
                .default_width(600.0)
                .show(ui.ctx(), |ui| {
                    ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                        ui.monospace(&self.last_preprompt);
                    });
                });
        }
    }
}
