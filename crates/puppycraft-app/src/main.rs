mod app;
mod chat_dock;
mod settings_view;
mod theme;
mod views_advisor;
mod views_care;
mod views_life;

use app::PuppyCraftApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1360.0, 860.0])
            .with_min_inner_size([960.0, 620.0])
            .with_title("PuppyCraft — Hundeberatung, Enzyklopädie & Hundsi-Werkstatt"),
        ..Default::default()
    };

    eframe::run_native(
        "PuppyCraft",
        options,
        Box::new(|cc| Ok(Box::new(PuppyCraftApp::new(cc)))),
    )
}
