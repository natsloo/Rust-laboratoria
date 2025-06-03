struct Recenzja {
    uzytkownik: String,
    ocena: u8,
    komentarz: String,
}
impl Recenzja {
    fn new(uzytkownik: String, ocena: u8, komentarz: String) -> Self {
        Self {
            uzytkownik,
            ocena,
            komentarz,
        }
    }
}
struct Ksiazka {
    tytul: String,
    autor: String,
    recenzje: Vec<Recenzja>,
}
impl Ksiazka {
    fn new(tytul: String, autor: String) -> Self {
        Self {
            tytul,
            autor,
            recenzje: vec![],
        }
    }
    fn dodaj_recenzje(&mut self, rec: Recenzja) {
        self.recenzje.push(rec);
    }
}

struct Biblioteka {
    ksiazki: Vec<Ksiazka>,
}
impl Biblioteka {
    fn new() -> Self {
        Self { ksiazki: vec![] }
    }
    fn dodaj_ksiazke(&mut self, k: Ksiazka) {
        self.ksiazki.push(k)
    }
    fn dodaj_recenzje(&mut self, tytul: &str, recenzja: Recenzja) {
        for k in &mut self.ksiazki {
            if k.tytul == tytul {
                k.dodaj_recenzje(recenzja);
                break;
            }
        }
    }
    fn srednia_ocena(&self, tytul: &str) -> Option<f32> {
        let mut suma = 0;
        let mut dlugosc = 0.0;
        for k in &self.ksiazki {
            if k.tytul == tytul {
                if k.recenzje.len() == 0 {
                    return None;
                }
                dlugosc = k.recenzje.len() as f32;
                for r in &k.recenzje {
                    suma += r.ocena;
                }
            }
        }
        Some(suma as f32 / dlugosc)
    }
    fn najlepiej_oceniane(&self, liczba: usize) -> Vec<String> {
        let mut v: Vec<(String, f32)> = vec![];
        let mut naj = vec![];
        for k in &self.ksiazki {
            v.push((k.tytul.clone(), self.srednia_ocena(&k.tytul).unwrap_or(0.0)));
        }
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for i in 0..liczba.min(v.len()) {
            naj.push(v[i].0.clone());
        }
        naj
    }
    fn komentarze_dla_autora(&self, autor: &str) -> Vec<String> {
        let mut kom = vec![];
        for k in &self.ksiazki {
            if k.autor == autor {
                for r in &k.recenzje {
                    kom.push(r.komentarz.clone())
                }
            }
        }
        kom
    }
}

fn main() {
    let mut b = Biblioteka::new();

    let k = Ksiazka::new("Mistrz i Małgorzata".to_string(), "Bułhakow".to_string());
    let k1 = Ksiazka::new("Zbrodnia i kara".to_string(), "Dostojewski".to_string());
    let k2 = Ksiazka::new("Rok 1984".to_string(), "Orwell".to_string());
    let k3 = Ksiazka::new("Proces".to_string(), "Kafka".to_string());
    let k4 = Ksiazka::new("Lalka".to_string(), "Prus".to_string());
    b.dodaj_ksiazke(k);
    b.dodaj_ksiazke(k1);
    b.dodaj_ksiazke(k2);
    b.dodaj_ksiazke(k3);
    b.dodaj_ksiazke(k4);

    b.dodaj_recenzje(
        "Mistrz i Małgorzata", Recenzja::new("Anna".to_string(), 5, "Arcydzieło!".to_string()),
    );
    b.dodaj_recenzje(
        "Mistrz i Małgorzata", Recenzja::new("Jan".to_string(), 4, "Trochę dziwne, ale dobre".to_string()),
    );
    b.dodaj_recenzje(
        "Zbrodnia i kara", Recenzja::new("Karol".to_string(), 5, "Poruszająca historia".to_string()),
    );
    b.dodaj_recenzje(
        "Zbrodnia i kara", Recenzja::new("Ala".to_string(), 4, "Ciężka, ale ważna książka".to_string(),
        ),
    );
    b.dodaj_recenzje(
        "Rok 1984", Recenzja::new("Monika".to_string(), 5, "Przerażająco aktualne".to_string()),
    );
    b.dodaj_recenzje(
        "Rok 1984", Recenzja::new("Bartek".to_string(), 5, "Klasyka dystopii".to_string()),
    );
    b.dodaj_recenzje(
        "Proces", Recenzja::new("Tomek".to_string(), 3, "Nie do końca zrozumiałem".to_string(),
        ),
    );
    b.dodaj_recenzje("Lalka", Recenzja::new("Kasia".to_string(), 4, "Długa, ale wciągająca".to_string()),
    );
    b.dodaj_recenzje("Lalka", Recenzja::new("Ola".to_string(), 3, "Nie moja bajka".to_string()),
    );

    println!("Top 3: {:?}", b.najlepiej_oceniane(3));
    println!(
        "Komentarze dla Prusa: {:?}",
        b.komentarze_dla_autora("Prus")
    );
    println!(
        "Średnia ocena \"Lalki\": {:?}",
        b.srednia_ocena("Lalka").unwrap_or(0.0)
    );
}
