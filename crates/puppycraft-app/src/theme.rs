use egui::{Color32, CornerRadius, Frame, Margin, Pos2, RichText, Stroke, Ui, Vec2};

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg_canvas: Color32,
    pub bg_card: Color32,
    pub bg_card_alt: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub good: Color32,
    pub warn: Color32,
    pub danger: Color32,
}

pub fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            bg_canvas: Color32::from_rgb(20, 18, 17),
            bg_card: Color32::from_rgb(31, 28, 26),
            bg_card_alt: Color32::from_rgb(40, 36, 33),
            border: Color32::from_rgb(62, 55, 50),
            text_primary: Color32::from_rgb(242, 236, 228),
            text_muted: Color32::from_rgb(168, 158, 148),
            accent: Color32::from_rgb(232, 158, 66),
            accent_soft: Color32::from_rgba_unmultiplied(232, 158, 66, 36),
            good: Color32::from_rgb(88, 194, 132),
            warn: Color32::from_rgb(235, 176, 68),
            danger: Color32::from_rgb(226, 94, 94),
        }
    } else {
        Palette {
            bg_canvas: Color32::from_rgb(249, 245, 238),
            bg_card: Color32::from_rgb(255, 253, 249),
            bg_card_alt: Color32::from_rgb(242, 235, 224),
            border: Color32::from_rgb(219, 208, 193),
            text_primary: Color32::from_rgb(38, 32, 28),
            text_muted: Color32::from_rgb(112, 100, 90),
            accent: Color32::from_rgb(198, 108, 34),
            accent_soft: Color32::from_rgba_unmultiplied(198, 108, 34, 28),
            good: Color32::from_rgb(42, 146, 88),
            warn: Color32::from_rgb(196, 128, 22),
            danger: Color32::from_rgb(198, 56, 56),
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, dark: bool) {
    let pal = palette(dark);
    let theme = if dark {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    ctx.set_theme(theme);
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.override_text_color = Some(pal.text_primary);
    visuals.weak_text_color = Some(pal.text_muted);
    visuals.panel_fill = pal.bg_canvas;
    visuals.window_fill = pal.bg_card;
    visuals.extreme_bg_color = pal.bg_card_alt;
    visuals.faint_bg_color = pal.bg_card_alt;
    visuals.selection.bg_fill = pal.accent_soft;
    visuals.selection.stroke = Stroke::new(1.0, pal.accent);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, pal.text_primary);
    visuals.widgets.noninteractive.bg_fill = pal.bg_card;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, pal.border);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, pal.text_primary);
    visuals.widgets.inactive.bg_fill = pal.bg_card_alt;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, pal.border);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(7);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, pal.text_primary);
    visuals.widgets.hovered.bg_fill = pal.accent_soft;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, pal.accent);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(7);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5, pal.text_primary);
    visuals.widgets.active.bg_fill = pal.accent;
    visuals.widgets.active.corner_radius = CornerRadius::same(7);
    visuals.window_corner_radius = CornerRadius::same(12);
    visuals.window_stroke = Stroke::new(1.0, pal.border);
    ctx.set_visuals_of(theme, visuals);

    let mut style = (*ctx.style_of(theme)).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 7.0);
    style.spacing.button_padding = Vec2::new(11.0, 6.0);
    style.spacing.window_margin = Margin::same(14);
    ctx.set_style_of(theme, style);
}

pub fn card_frame(_ui: &Ui, dark: bool) -> Frame {
    let pal = palette(dark);
    Frame::new()
        .fill(pal.bg_card)
        .stroke(Stroke::new(1.0, pal.border))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(12))
}

pub fn hero_header(ui: &mut Ui, dark: bool, title: &str, subtitle: &str) {
    let pal = palette(dark);
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(4.0, 36.0), egui::Sense::hover());
        ui.painter()
            .rect_filled(rect, CornerRadius::same(2), pal.accent);
        ui.vertical(|ui| {
            ui.label(
                RichText::new(title)
                    .size(21.0)
                    .strong()
                    .color(pal.text_primary),
            );
            ui.label(RichText::new(subtitle).size(12.5).color(pal.text_muted));
        });
    });
    ui.add_space(6.0);
}

pub fn pill_badge(ui: &mut Ui, text: &str, bg: Color32, fg: Color32) {
    Frame::new()
        .fill(bg)
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(9, 3))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.5).strong().color(fg));
        });
}

pub fn stat_bar(ui: &mut Ui, dark: bool, label: &str, value: u8, max: u8, color: Color32) {
    let pal = palette(dark);
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new(145.0, 18.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(RichText::new(label).size(12.5).color(pal.text_muted));
            },
        );
        let (rect, _) = ui.allocate_exact_size(Vec2::new(120.0, 10.0), egui::Sense::hover());
        let p = ui.painter();
        p.rect_filled(rect, CornerRadius::same(5), pal.bg_card_alt);
        let frac = (f32::from(value.min(max)) / f32::from(max.max(1))).clamp(0.0, 1.0);
        if frac > 0.0 {
            let fill_rect =
                egui::Rect::from_min_size(rect.min, Vec2::new(rect.width() * frac, rect.height()));
            p.rect_filled(fill_rect, CornerRadius::same(5), color);
        }
        ui.label(
            RichText::new(format!("{value}/{max}"))
                .size(11.5)
                .strong()
                .color(pal.text_primary),
        );
    });
}

pub fn draw_paw_emblem(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    let p = ui.painter();
    let c = rect.center();
    let r = size * 0.45;
    // Main pad
    p.circle_filled(Pos2::new(c.x, c.y + r * 0.24), r * 0.34, color);
    // 4 toes
    let toes = [(-0.46, -0.18), (-0.18, -0.44), (0.18, -0.44), (0.46, -0.18)];
    for (dx, dy) in toes {
        p.circle_filled(Pos2::new(c.x + r * dx, c.y + r * dy), r * 0.16, color);
    }
}

#[derive(Clone, Copy)]
enum EarStyle {
    Floppy,
    Upright,
    LongDroop,
}

fn breed_visual_style(breed_id: &str) -> (Color32, Color32, EarStyle) {
    match breed_id {
        "golden-retriever" => (
            Color32::from_rgb(224, 178, 96),
            Color32::from_rgb(245, 214, 148),
            EarStyle::Floppy,
        ),
        "labrador-retriever" => (
            Color32::from_rgb(218, 185, 118),
            Color32::from_rgb(238, 212, 160),
            EarStyle::Floppy,
        ),
        "deutscher-schaeferhund" => (
            Color32::from_rgb(176, 122, 62),
            Color32::from_rgb(56, 46, 40),
            EarStyle::Upright,
        ),
        "border-collie" | "australian-shepherd" => (
            Color32::from_rgb(52, 50, 56),
            Color32::from_rgb(242, 240, 236),
            EarStyle::Floppy,
        ),
        "berner-sennenhund" | "cavalier-king-charles" => (
            Color32::from_rgb(48, 44, 44),
            Color32::from_rgb(198, 120, 64),
            EarStyle::LongDroop,
        ),
        "dackel" | "vizsla" | "rhodesian-ridgeback" | "boxer" => (
            Color32::from_rgb(168, 92, 46),
            Color32::from_rgb(198, 128, 76),
            EarStyle::Floppy,
        ),
        "bichon-frise" | "malteser" | "samojede" => (
            Color32::from_rgb(242, 238, 232),
            Color32::from_rgb(252, 250, 246),
            EarStyle::Floppy,
        ),
        "shiba-inu" => (
            Color32::from_rgb(212, 122, 54),
            Color32::from_rgb(246, 236, 220),
            EarStyle::Upright,
        ),
        "cocker-spaniel" => (
            Color32::from_rgb(186, 114, 58),
            Color32::from_rgb(218, 152, 92),
            EarStyle::LongDroop,
        ),
        "franzoesische-bulldogge" | "mops" => (
            Color32::from_rgb(206, 182, 146),
            Color32::from_rgb(58, 50, 46),
            EarStyle::Upright,
        ),
        "zwergpudel" | "grosspudel" => (
            Color32::from_rgb(188, 112, 64),
            Color32::from_rgb(214, 142, 92),
            EarStyle::LongDroop,
        ),
        _ => (
            Color32::from_rgb(182, 144, 102),
            Color32::from_rgb(224, 196, 158),
            EarStyle::Floppy,
        ),
    }
}

/// GPU-rendered breed portrait card showing Puppy ("Welpe") or Adult ("Erwachsen")
pub fn draw_breed_portrait(ui: &mut Ui, dark: bool, breed_id: &str, is_puppy: bool) {
    let pal = palette(dark);
    let (coat, accent, ears) = breed_visual_style(breed_id);
    card_frame(ui, dark).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::new(132.0, 106.0), egui::Sense::hover());
            let p = ui.painter();
            let bg = if is_puppy {
                pal.accent_soft
            } else {
                pal.bg_card_alt
            };
            p.rect_filled(rect, CornerRadius::same(8), bg);

            let cx = rect.center().x;
            let cy = rect.center().y + if is_puppy { 4.0 } else { 0.0 };
            let head_r = if is_puppy { 25.0 } else { 23.0 };

            // Body / chest
            let body_w = if is_puppy { 26.0 } else { 32.0 };
            let body_h = if is_puppy { 20.0 } else { 26.0 };
            p.circle_filled(Pos2::new(cx, cy + 28.0), body_w * 0.72, coat);
            p.circle_filled(Pos2::new(cx, cy + 26.0), body_h * 0.45, accent);

            // Collar for adult
            if !is_puppy {
                p.line_segment(
                    [
                        Pos2::new(cx - 16.0, cy + 16.0),
                        Pos2::new(cx + 16.0, cy + 16.0),
                    ],
                    Stroke::new(4.0, pal.accent),
                );
            }

            // Ears
            match ears {
                EarStyle::Upright => {
                    let l_ear = [
                        Pos2::new(cx - head_r * 0.75, cy - 6.0),
                        Pos2::new(cx - head_r * 0.95, cy - 34.0),
                        Pos2::new(cx - head_r * 0.25, cy - 16.0),
                    ];
                    let r_ear = [
                        Pos2::new(cx + head_r * 0.75, cy - 6.0),
                        Pos2::new(cx + head_r * 0.95, cy - 34.0),
                        Pos2::new(cx + head_r * 0.25, cy - 16.0),
                    ];
                    p.add(egui::Shape::convex_polygon(
                        l_ear.to_vec(),
                        coat,
                        Stroke::NONE,
                    ));
                    p.add(egui::Shape::convex_polygon(
                        r_ear.to_vec(),
                        coat,
                        Stroke::NONE,
                    ));
                }
                EarStyle::Floppy => {
                    p.circle_filled(Pos2::new(cx - head_r * 0.88, cy + 2.0), 10.5, accent);
                    p.circle_filled(Pos2::new(cx + head_r * 0.88, cy + 2.0), 10.5, accent);
                }
                EarStyle::LongDroop => {
                    p.rect_filled(
                        egui::Rect::from_center_size(
                            Pos2::new(cx - head_r * 0.92, cy + 8.0),
                            Vec2::new(13.0, 30.0),
                        ),
                        CornerRadius::same(6),
                        accent,
                    );
                    p.rect_filled(
                        egui::Rect::from_center_size(
                            Pos2::new(cx + head_r * 0.92, cy + 8.0),
                            Vec2::new(13.0, 30.0),
                        ),
                        CornerRadius::same(6),
                        accent,
                    );
                }
            }

            // Head
            p.circle_filled(Pos2::new(cx, cy - 4.0), head_r, coat);
            // Muzzle
            let muzzle_r = if is_puppy { 10.0 } else { 12.0 };
            p.circle_filled(Pos2::new(cx, cy + 6.0), muzzle_r, accent);
            // Eyes (puppies have bigger rounder eyes)
            let eye_r = if is_puppy { 3.8 } else { 3.0 };
            p.circle_filled(
                Pos2::new(cx - 8.5, cy - 8.0),
                eye_r,
                Color32::from_rgb(24, 20, 18),
            );
            p.circle_filled(
                Pos2::new(cx + 8.5, cy - 8.0),
                eye_r,
                Color32::from_rgb(24, 20, 18),
            );
            // Eye catchlights
            p.circle_filled(Pos2::new(cx - 9.5, cy - 9.0), 1.1, Color32::WHITE);
            p.circle_filled(Pos2::new(cx + 7.5, cy - 9.0), 1.1, Color32::WHITE);
            // Nose
            p.circle_filled(
                Pos2::new(cx, cy + 4.0),
                if is_puppy { 3.6 } else { 4.4 },
                Color32::from_rgb(30, 26, 26),
            );
            // Puppy tongue
            if is_puppy {
                p.circle_filled(
                    Pos2::new(cx, cy + 12.5),
                    3.0,
                    Color32::from_rgb(232, 118, 128),
                );
            }

            ui.add_space(4.0);
            ui.label(
                RichText::new(if is_puppy { "Welpe" } else { "Erwachsen" })
                    .strong()
                    .size(12.5)
                    .color(if is_puppy {
                        pal.accent
                    } else {
                        pal.text_primary
                    }),
            );
        });
    });
}
