use eframe::egui;

/// Reprezentuje możliwe statusy zadania w cyklu życia
#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    DoZrobienia,
    WTrakcie,
    Zrobione,
}

/// Główna struktura reprezentująca pojedyncze zadanie.
struct Zadanie {
    /// Unikalny identyfikator, kluczowy dla Drag&Drop.
    id: usize,
    tresc: String,
    status: Status,
}

/// Bufor Akcji.
enum Akcja {
    Usun(usize),
    ZmienStatus(usize, Status),
    DodajNowe,
}

/// Główny stan aplikacji Kanban.
#[derive(Default)]
struct KanbanApp {
    tresc_zadania: String,
    lista_zadan: Vec<Zadanie>,
    next_id: usize,
}

impl eframe::App for KanbanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Styl globalny - ciemny motyw
        ctx.set_visuals(egui::Visuals::dark());

        // Kolejka akcji do wykonania PO narysowaniu klatki
        let mut akcje: Vec<Akcja> = Vec::new();

        // Panel boczny
        egui::SidePanel::left("lewy_panel")
            .resizable(false) // Blokada zmiany szerokości
            .exact_width(200.0)
            .show(ctx, |ui| {
                ui.add_space(20.0); // Margines od góry
                ui.heading("Menu");
                ui.add_space(20.0);

                // Sekcja: Nowe Zadanie
                ui.group(|ui| {
                    ui.label("Nowe zadanie:");
                    let input = ui.text_edit_singleline(&mut self.tresc_zadania);
                    ui.add_space(5.0);

                    // Przycisk rozciągnięty na całą szerokość panelu
                    let btn = ui.add_sized(
                        [ui.available_width(), 30.0],
                        egui::Button::new(
                            egui::RichText::new("Dodaj Task")
                                .color(egui::Color32::BLACK)
                                .strong(),
                        )
                        .fill(egui::Color32::from_rgb(100, 149, 237))
                        .min_size(egui::vec2(0.0, 30.0)),
                    );

                    if (btn.clicked()
                        || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                        && !self.tresc_zadania.is_empty()
                    {
                        akcje.push(Akcja::DodajNowe);
                        // Utrzymanie focusa
                        input.request_focus();
                    }
                });

                // Sekcja: Przycisk wyjścia
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    if ui
                        .add_sized(
                            [ui.available_width(), 40.0],
                            egui::Button::new("Quit").fill(egui::Color32::from_rgb(180, 40, 40)),
                        )
                        .clicked()
                    {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });

        // Panel centralny
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Kanban Board");
            ui.separator();
            ui.add_space(20.0);

            // Podział ekranu na 3 kolumny
            ui.columns(3, |kolumny| {
                let stany = [Status::DoZrobienia, Status::WTrakcie, Status::Zrobione];
                let tytuly = ["Do Zrobienia", "W Trakcie", "Zrobione"];

                for (i, status) in stany.iter().enumerate() {
                    let ui_col = &mut kolumny[i];

                    // Ramka dla kolumny
                    egui::Frame::group(ui_col.style())
                        .fill(egui::Color32::TRANSPARENT)
                        .corner_radius(10) // Zaokrąglenie rogów kolumny
                        .inner_margin(10.0)
                        .show(ui_col, |ui| {
                            // Nagłówek kolumny
                            ui.vertical_centered(|ui| {
                                ui.heading(tytuly[i]);
                            });
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);

                            // --- STREFA DROP ZONE ---
                            let (_, payload) =
                                ui.dnd_drop_zone::<usize, _>(egui::Frame::NONE, |ui| {
                                    // Filtrujemy zadania
                                    let zadania_w_kolumnie: Vec<&Zadanie> = self
                                        .lista_zadan
                                        .iter()
                                        .filter(|z| z.status == *status)
                                        .collect();

                                    for zadanie in zadania_w_kolumnie {
                                        let item_id = egui::Id::new("task").with(zadanie.id);

                                        // Element przeciągany (Drag Source)
                                        ui.dnd_drag_source(item_id, zadanie.id, |ui| {
                                            // Wygląd pojedynczego kafelka zadania
                                            egui::Frame::group(ui.style())
                                                .corner_radius(5)
                                                .fill(egui::Color32::from_rgb(160, 180, 240))
                                                .inner_margin(8.0)
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(&zadanie.tresc)
                                                                .color(egui::Color32::BLACK),
                                                        );
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        if ui.add(
                                                            egui::Button::new(
                                                                egui::RichText::new("Usuń")
                                                                    .size(12.0)
                                                                    .color(egui::Color32::BLACK)
                                                            )
                                                            .fill(egui::Color32::from_gray(230))
                                                            .min_size(egui::vec2(40.0, 20.0))
                                                            .corner_radius(4.0)
                                                        ).clicked() {
                                                            akcje.push(Akcja::Usun(zadanie.id));
                                                        }
                                                    });
                                                });
                                            });
                                        });
                                        ui.add_space(8.0); // Odstęp między kafelkami
                                    }
                                    // Rezerwacja przestrzeni dla Drop Zone
                                    ui.allocate_space(ui.available_size());
                                });

                            if let Some(upuszczone_id) = payload {
                                akcje.push(Akcja::ZmienStatus(*upuszczone_id, *status));
                            }
                        });
                }
            });
        });

        // Wykonanie zebranych akcji
        for akcja in akcje {
            match akcja {
                Akcja::DodajNowe => {
                    self.lista_zadan.push(Zadanie {
                        id: self.next_id,
                        tresc: self.tresc_zadania.clone(),
                        status: Status::DoZrobienia,
                    });
                    self.next_id += 1;
                    self.tresc_zadania.clear();
                }
                Akcja::Usun(id) => {
                    self.lista_zadan.retain(|z| z.id != id);
                }
                Akcja::ZmienStatus(id, nowy_status) => {
                    if let Some(z) = self.lista_zadan.iter_mut().find(|z| z.id == id) {
                        z.status = nowy_status;
                    }
                }
            }
        }
    }
}

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
