use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Moje Pierwsze Okno", 
        eframe::NativeOptions::default(), 
        Box::new(|_cc| Ok(Box::new(MojaAplikacja))) // To jest boilerplate (nieunikniony w Rust)
    )
}

struct MojaAplikacja;

impl eframe::App for MojaAplikacja {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tablica Kanban!");

            ui.horizontal(|ui| {
                egui::Frame::group(ui.style())
                .corner_radius(20.0)
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2(200.0, 400.0));
                    ui.label("ToDo");
                });

                egui::Frame::group(ui.style())
                .corner_radius(20.0)
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2(200.0, 400.0));
                    ui.label("Doing");
                });

            });
        });
    }
}