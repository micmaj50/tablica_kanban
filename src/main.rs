use eframe::egui;

mod models;
mod app;

use app::KanbanApp;

fn main() -> eframe::Result<()> {
    // Konfiguracja kontekstu okna
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0]) // Startowy rozmiar okna
            .with_active(true),
        ..Default::default()
    };

    eframe::run_native(
        "Kanban Rust",
        options,
        Box::new(|_cc| Ok(Box::new(KanbanApp::default()))),
    )
}
