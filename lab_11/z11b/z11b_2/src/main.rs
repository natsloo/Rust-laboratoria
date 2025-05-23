#[derive(PartialEq, Debug)]
enum Rezultat {
    Rozwiazane,
    Odrzucone,
    Przedawnione,
    ZaMaloInformacji,
    Inne(String),
}
#[derive(PartialEq, Debug)]
enum Status {
    Otwarte,
    Przetwarzane,
    Zamkniete(Rezultat)
}
#[derive(Debug)]
struct Zgloszenie {
    id: u16,
    tytul: String,
    status: Status
}

impl Zgloszenie {
    fn new(id: u16, tytul: String, status: Status) -> Zgloszenie {
        Zgloszenie {
            id,
            tytul,
            status
        }
    }
    fn set_status(&mut self, status: Status){
        self.status = status;
    }
}

fn filtruj_po_statusie(zgloszenia: Vec<Zgloszenie>, status: Status) -> Vec<Zgloszenie> {
    let mut wybrane = Vec::new();
    for z in zgloszenia {
        if z.status == status {
            wybrane.push(z);
        }
    }
    wybrane
}

fn main() {
    let id = 0;
    let mut zgloszenia = Vec::new();
    let z = Zgloszenie::new(id, "Awaria klimatyzacji".to_string(), Status::Otwarte);
    zgloszenia.push(z);
    let z1 = Zgloszenie::new(id + 1, "Awaria lodowki".to_string(), Status::Przetwarzane);
    zgloszenia.push(z1);
    let z2 = Zgloszenie::new(id + 2, "Awaria telewizora".to_string(), Status::Zamkniete(Rezultat::Rozwiazane));
    zgloszenia.push(z2);
    let z3 = Zgloszenie::new(id + 3, "Awaria pralki".to_string(), Status::Otwarte);
    zgloszenia.push(z3);
    println!("{:?}", filtruj_po_statusie(zgloszenia, Status::Otwarte));
}
