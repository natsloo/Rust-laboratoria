#[derive (PartialEq)]
enum KlasaPodrozy {
    Ekonomiczna,
    Biznesowa,
    Pierwsza
}
#[derive (PartialEq)]
enum Status {
    Zarezerwowane,
    Odwolane
}
#[derive (PartialEq)]
struct Pasazer {
    imie: String,
    nazwisko: String
}

impl Pasazer {
    fn new(imie: String, nazwisko: String) -> Self {
        Pasazer {
            imie,
            nazwisko
        }
    }
    fn imie_nazwisko(&self) -> String {
        format!("{} {}", self.imie, self.nazwisko)
    }
}

struct Rezerwacja {
    pasazer: Pasazer,
    numer_lotu: String,
    klasa_podrozy: KlasaPodrozy,
    status: Status
}

impl Rezerwacja {
    fn new(pasazer: Pasazer, numer_lotu: String, klasa_podrozy: KlasaPodrozy, status: Status) -> Self {
        Self {
            pasazer,
            numer_lotu,
            klasa_podrozy,
            status
        }
    }
}

struct SystemRezerwacji {
    rezerwacje: Vec<Rezerwacja>
}

impl SystemRezerwacji {
    fn new() -> Self {
        SystemRezerwacji {
            rezerwacje: Vec::new()
        }
    }
    fn dodaj_rezerwacje(&mut self, r: Rezerwacja) {
        self.rezerwacje.push(r);
    }
    fn anuluj(&mut self, p: Pasazer) {
        for mut r in &mut self.rezerwacje {
            if r.pasazer == p {
                r.status = Status::Odwolane;
            }
        }
    }
    fn zlicz_po_klasie(&self, kp: KlasaPodrozy) -> usize {
        let mut wybrane = Vec::new();
        for r in &self.rezerwacje {
            if r.klasa_podrozy == kp {
                wybrane.push(r);
            }
        }
        wybrane.len()
    }
    fn zlicz_po_statusie(&self, s: Status) -> usize {
        let mut wybrane = Vec::new();
        for r in &self.rezerwacje {
            if r.status == s {
                wybrane.push(r);
            }
        }
        wybrane.len()
    }
    fn zlicz_po_klasie_i_statusie(&self, kp: KlasaPodrozy, s: Status) -> usize {
        let mut wybrane = Vec::new();
        for r in &self.rezerwacje {
            if r.klasa_podrozy == kp && r.status == s {
                wybrane.push(r);
            }
        }
        wybrane.len()
    }
    fn pasazerowie_w_klasie(&self, kp: KlasaPodrozy) -> Vec<String> {
        let mut pasazerowie = Vec::new();
        let mut wybrane = Vec::new();
        for r in &self.rezerwacje {
            if r.klasa_podrozy == kp {
                wybrane.push(r);
            }
        }
        for r in wybrane {
            pasazerowie.push(r.pasazer.imie_nazwisko());
        }
        pasazerowie.sort();
        pasazerowie
    }
}



fn main() {
    let mut s = SystemRezerwacji::new();
    let p = Pasazer::new("Jan".to_string(),"Nowak".to_string());
    let p1 = Pasazer::new("Adam".to_string(),"Kowalski".to_string());
    let p2 = Pasazer::new("Leonard".to_string(),"Drążkiewicz".to_string());
    let p3 = Pasazer::new("Remigiusz".to_string(),"Potocki".to_string());
    let r = Rezerwacja::new(p,"MH370".to_string(), KlasaPodrozy::Pierwsza, Status::Odwolane);
    let r1 = Rezerwacja::new(p1, "LO123".to_string(), KlasaPodrozy::Ekonomiczna, Status::Zarezerwowane);
    let r2 = Rezerwacja::new(p2, "FR456".to_string(), KlasaPodrozy::Biznesowa, Status::Zarezerwowane);
    let r3 = Rezerwacja::new(p3, "AF202".to_string(), KlasaPodrozy::Biznesowa, Status::Zarezerwowane);
    s.dodaj_rezerwacje(r);
    s.dodaj_rezerwacje(r1);
    s.dodaj_rezerwacje(r2);
    s.dodaj_rezerwacje(r3);
    println!("{}", s.zlicz_po_klasie_i_statusie(KlasaPodrozy::Biznesowa, Status::Zarezerwowane));
    println!("{:?}", s.pasazerowie_w_klasie(KlasaPodrozy::Biznesowa));
}
