enum Blad {
    BrakBledu,
    ZlyFormatPliku,
    PlikNieIstnieje(String),
    PlikZbytDuzy(f64,f64),
}

impl Blad {
    fn pokaz_komunikat(&self) {
        match &self{
            Blad::BrakBledu => println!("Nie ma bledu :)"),
            Blad::ZlyFormatPliku => println!("Zly format pliku!"),
            Blad::PlikNieIstnieje(s) => println!("Plik {} nie istnieje!", s),
            Blad::PlikZbytDuzy(ac,max) => println!("Plik jest zbyt duzy! Rozmiar pliku = {} MG, max rozmiar = {} MG, ", ac, max),
        }
    }
}

fn main() {
    let b = Blad::BrakBledu;
    b.pokaz_komunikat();
    let b = Blad::ZlyFormatPliku;
    b.pokaz_komunikat();
    let s = "dane/goscie.csv".to_string();
    let b = Blad::PlikNieIstnieje(s);
    b.pokaz_komunikat();
    let b = Blad::PlikZbytDuzy(27.3,10.0);
    b.pokaz_komunikat();
}
