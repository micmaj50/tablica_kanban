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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    
                    // Definiujemy wygląd ramki kolumny
                    let frame = egui::Frame::group(ui_col.style())
                        .inner_margin(5.0)
                        .fill(ui_col.visuals().faint_bg_color); // Lekkie tło dla kolumny

                    // Tworzymy funkcję rysującą kontener, który wykrywa, czy coś na niego upuszczamy.
                    // <usize> oznacza, że oczekujemy, iż spadnie tu liczba (ID zadania).
                    let response = ui_col.dnd_drop_zone::<usize, _>(frame, |ui| {
                        
                        ui.heading(tytuly[i]);
                        ui.separator();

                        // Filtrowanie zadań dla tej kolumny. Zbieramy je w nową kolekcję
                        let zadania_w_kolumnie: Vec<&Zadanie> = self.lista_zadan
                            .iter()
                            .filter(|z| z.status == *status)
                            .collect();

                        for zadanie in zadania_w_kolumnie {
                            // Generujemy unikalne ID dla UI (wymagane przez egui)
                            let item_id = egui::Id::new("task").with(zadanie.id);

                            // Tworzymy ELEMENT PRZESUWNY
                            // item_id = ID elementu UI
                            // zadanie.id = Payload (to co niesiemy, czyli ID z bazy danych)
                            ui.dnd_drag_source(item_id, zadanie.id, |ui| {
                                // Wygląd pojedynczego kafelka
                                ui.group(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("::"); // Uchwyt do łapania (estetyka)
                                        ui.label(&zadanie.tresc);
                                        
                                        // Przycisk usuwania (czerwony)
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.small_button("🗑").clicked() {
                                                akcje.push(Akcja::Usun(zadanie.id));
                                            }
                                        });
                                    });
                                });
                            });
                            ui.add_space(5.0);
                        }
                    });

                    // Obsługa UPUSZCZENIA
                    // Sprawdzamy, czy w tej klatce coś spadło na tę kolumnę
                    if let Some(upuszczone_id) = response.1 {
                        // Jeśli tak, dodajemy akcję zmiany statusu!
                        akcje.push(Akcja::ZmienStatus(*upuszczone_id, *status));
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
