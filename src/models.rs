// Reprezentuje możliwe statusy zadania w cyklu życia
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status {
    DoZrobienia,
    WTrakcie,
    Zrobione,
}

// Główna struktura reprezentująca pojedyncze zadanie.
pub struct Zadanie {
    // Unikalny identyfikator, kluczowy dla Drag&Drop.
    pub id: usize,
    pub tresc: String,
    pub status: Status,
}

// Bufor Akcji.
pub enum Akcja {
    Usun(usize),
    ZmienStatus(usize, Status),
    DodajNowe,
}