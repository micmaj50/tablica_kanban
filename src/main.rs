use eframe::egui;

/// Reprezentuje możliwe statusy zadania w cyklu życia (workflow).
/// Odpowiada kolumnom na tablicy Kanban.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    /// Zadanie oczekujące na realizację.
    DoZrobienia,
    /// Zadanie aktualnie wykonywane.
    WTrakcie,
    /// Zadanie zakończone.
    Zrobione,
}

/// Główna struktura reprezentująca pojedyncze zadanie.
struct Zadanie {
    /// Unikalny identyfikator, kluczowy dla Drag&Drop.
    id: usize,
    /// Treść/opis zadania wprowadzony przez użytkownika.
    tresc: String,
    /// Aktualny stan zadania (przypisanie do kolumny).
    status: Status,
}

/// Bufor Akcji.
enum Akcja {
    /// Usunięcie zadania o podanym ID.
    Usun(usize),
    /// Zmiana statusu zadania (np. przesunięcie do innej kolumny).
    ZmienStatus(usize, Status),
}

/// Główny stan aplikacji Kanban.
/// Przechowuje wszystkie dane i konfigurację widoku.
struct KanbanApp {
    /// Bufor tekstowy dla pola wprowadzania nowego zadania.
    tresc_zadania: String,
    /// Wektor przechowywujący wszystkie zadania (niezależnie od statusu).
    lista_zadan: Vec<Zadanie>,
    /// Licznik służący do generowania unikalnych ID (auto-increment).
    next_id: usize,
}

impl Default for KanbanApp {
    fn default() -> Self {
        Self {
            tresc_zadania: String::new(),
            lista_zadan: Vec::new(),
            next_id: 0,
        }
    }
}

impl eframe::App for KanbanApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // --- NAGŁÓWEK ---
            ui.heading("Aplikacja Kanban");
            ui.separator();

            // --- DODAWANIE ZADANIA ---
            ui.horizontal(|ui| {
                ui.label("Dodaj zadanie: ");
                // Input pola tekstowego powiązany ze zmienną w structurze
                let input = ui.text_edit_singleline(&mut self.tresc_zadania);

                // Wykrywanie intencji użytkownika (wciśnięcie Enter lub kliknięcie przycisku)
                let enter_pressed = input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                let button_clicked = ui.button("Dodaj").clicked();

                // Walidacja i dodawanie nowego zadania
                if (button_clicked || enter_pressed) && !self.tresc_zadania.is_empty() {
                    self.lista_zadan.push(Zadanie {
                        id: self.next_id,
                        tresc: self.tresc_zadania.clone(),
                        status: Status::DoZrobienia, // Domyślnie wpada do pierwszej kolumny
                    });

                    // Reset stanu inputu
                    self.next_id += 1;
                    self.tresc_zadania.clear();

                    // Utrzymanie focusa na polu tekstowym dla seryjnego dodawania
                    input.request_focus(); 
                }
            });
            ui.add_space(10.0);
            ui.separator();

            // --- BUFOR AKCJI ---
            // Zbieramy tutaj co użytkownik klika, żeby zmienić to PO narysowaniu UI.
            let mut akcje: Vec<Akcja> = Vec::new();            

            // --- KOLUMNY KANBAN ---
            // Dzielimy ekran na 3 równe części
            ui.columns(3, |kolumny| {
                // Iterujemy po naszych statusach i przypisujemy im kolumnę UI
                let stany = [Status::DoZrobienia, Status::WTrakcie, Status::Zrobione];
                let tytuly = ["Do Zrobienia", "W Trakcie", "Zrobione"];

                for (i, status) in stany.iter().enumerate() {
                    let ui_col = &mut kolumny[i]; // Bierzemy referencję do konkretnej kolumny UI
                    
                    ui_col.heading(tytuly[i]);
                    ui_col.separator();

                    // Filtrowanie i wyświetlanie zadań dla danej kolumny
                    for zadanie in self.lista_zadan.iter().filter(|z| z.status == *status) {
                        ui_col.group(|ui| {
                            ui.label(format!("#{} {}", zadanie.id, zadanie.tresc));
                            
                            ui.horizontal(|ui| {
                                // Logika przycisków "Przesuń"
                                match status {
                                    Status::DoZrobienia => {
                                        if ui.button("➡️").clicked() {
                                            akcje.push(Akcja::ZmienStatus(zadanie.id, Status::WTrakcie));
                                        }
                                    },
                                    Status::WTrakcie => {
                                        if ui.button("⬅️").clicked() {
                                            akcje.push(Akcja::ZmienStatus(zadanie.id, Status::DoZrobienia));
                                        }
                                        if ui.button("➡️").clicked() {
                                            akcje.push(Akcja::ZmienStatus(zadanie.id, Status::Zrobione));
                                        }
                                    },
                                    Status::Zrobione => {
                                        if ui.button("⬅️").clicked() {
                                            akcje.push(Akcja::ZmienStatus(zadanie.id, Status::WTrakcie));
                                        }
                                    }
                                }

                                // Przycisk usuwania (czerwony)
                                if ui.add(egui::Button::new("🗑").fill(egui::Color32::DARK_RED)).clicked() {
                                    akcje.push(Akcja::Usun(zadanie.id));
                                }
                            });
                        });
                        ui_col.add_space(5.0);
                    }
                }
            });

            // --- APLIKOWANIE ZMIAN ---
            // Dopiero teraz, kiedy UI jest narysowane i nikt nie "pożycza" listy zadań,
            // możemy ją modyfikować. To jest klucz do Rusta!
            for akcja in akcje {
                match akcja {
                    Akcja::Usun(id_do_usuniecia) => {
                        // `retain` usuwa elementy, które NIE spełniają warunku (czyli usuwamy pasujące ID)
                        self.lista_zadan.retain(|z| z.id != id_do_usuniecia);
                    },
                    Akcja::ZmienStatus(id, nowy_status) => {
                        // Szukamy zadania po ID i zmieniamy jego status
                        if let Some(zadanie) = self.lista_zadan.iter_mut().find(|z| z.id == id) {
                            zadanie.status = nowy_status;
                        }
                    }
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    // Konfiguracja kontekstu okna
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]), // Startowy rozmiar okna
        ..Default::default()
    };

    eframe::run_native(
        "Kanban Rust",
        options,
        Box::new(|_cc| Ok(Box::new(KanbanApp::default()))),
    )
}
