use std::collections::HashMap;
use std::fmt;
#[derive(PartialEq)]
enum Specjalizacja {
    Frontend,
    Backend,
    DevOps,
    QA,
    Manager,
}
#[derive(PartialEq, Hash, Clone, Eq, Debug)]
enum StatusZadania {
    Nowe,
    WTrakcie,
    Zakonczone,
    Odrzucone(String), // powód odrzucenia
}
impl StatusZadania {
    fn to_str(&self) -> &str {
        match &self {
            StatusZadania::Nowe => "Nowe",
            StatusZadania::WTrakcie => "W Trakcie",
            StatusZadania::Zakonczone => "Zakonczone",
            StatusZadania::Odrzucone(_String) => "Odrzucone",
        }
    }
}
impl fmt::Display for StatusZadania {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

struct CzlonekZespolu {
    imie: String,
    nazwisko: String,
    specjalizacja: Specjalizacja,
}

struct Zadanie {
    nazwa: String,
    przypisany: Option<String>, // nazwisko osoby
    status: StatusZadania,
    godziny_oszacowane: u32,
    godziny_rzeczywiste: Option<u32>,
}
impl fmt::Display for Zadanie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Nazwa: {}, przypisany: {}, status zadania: {}, godziny oszacowane: {}, godziny rzeczywiste: {}.",
            self.nazwa,
            self.przypisany.clone().unwrap_or("NIKT".to_string()),
            self.status,
            self.godziny_oszacowane,
            self.godziny_rzeczywiste
                .map_or("BRAK".to_string(), |g| g.to_string())
        )
    }
}
struct Projekt {
    id: u32,
    nazwa: String,
    zespol: Vec<CzlonekZespolu>,
    zadania: Vec<Zadanie>,
}
impl Projekt {
    fn dodaj_czlonka(&mut self, czlonek: CzlonekZespolu) {
        self.zespol.push(czlonek)
    }
    fn dodaj_zadanie(&mut self, zadanie: Zadanie) {
        self.zadania.push(zadanie)
    }
    fn przypisz_zadanie(&mut self, nazwa_zadania: &str, nazwisko: &str) -> Result<(), String> {
        for z in &mut self.zadania {
            if z.nazwa == nazwa_zadania {
                if z.przypisany.is_none() {
                    z.przypisany = Some(nazwisko.to_string());
                    return Ok(());
                } else {
                    return Err("To zadanie jest juz przypisane!".to_string());
                }
            }
        }
        Err("Nie ma takiego zadania!".to_string())
    }
    fn zakoncz_zadanie(&mut self, nazwa: &str, godziny: u32) -> Result<(), String> {
        for z in &mut self.zadania {
            if z.nazwa == nazwa {
                if z.status != StatusZadania::WTrakcie && z.status != StatusZadania::Nowe {
                    return Err("Nieodpowiedni status zadania!".to_string());
                } else if !z.godziny_rzeczywiste.is_none() {
                    return Err("Godziny rzeczywiste juz zostaly wprowadzone!".to_string());
                } else {
                    z.status = StatusZadania::Zakonczone;
                    z.godziny_rzeczywiste = Some(godziny);
                    return Ok(());
                }
            }
        }
        Err("Nie ma takiego zadania!".to_string())
    }
    fn odrzuc_zadanie(&mut self, nazwa: &str, powod: String) -> Result<(), String> {
        for z in &mut self.zadania {
            if z.nazwa == nazwa {
                if z.status != StatusZadania::Nowe {
                    return Err("Nieodpowiedni status zadania!".to_string());
                } else {
                    z.status = StatusZadania::Odrzucone(powod.clone());
                    return Ok(());
                }
            }
        }
        Err("Nie ma takiego zadania!".to_string())
    }
    fn godziny_na_osobe(&self) -> HashMap<String, u32> {
        let mut hm: HashMap<String, u32> = HashMap::new();
        for z in &self.zadania {
            if let Some(ref _nazwisko) = z.przypisany {
                hm.entry(z.przypisany.clone().unwrap())
                    .and_modify(|g_o| *g_o += z.godziny_oszacowane)
                    .or_insert(z.godziny_oszacowane);
            }
        }
        hm
    }
    fn zadania_dla_specjalizacji(&self, spec: Specjalizacja) -> Vec<String> {
        let mut nazwy: Vec<String> = vec![];
        for z in &self.zadania {
            if let Some(ref _nazwisko) = z.przypisany {
                for c in &self.zespol {
                    if z.przypisany.clone().unwrap() == c.nazwisko {
                        if c.specjalizacja == spec {
                            nazwy.push(z.nazwa.clone())
                        }
                    }
                }
            }
        }
        nazwy
    }
    fn raport(&self) {
        for z in &self.zadania {
            println!("{z}")
        }
    }
    fn przeciazony_pracownik(&self) -> Option<String> {
        for z in &self.zadania {
            if let Some(ref _nazwisko) = z.przypisany {
                for _c in &self.zespol {
                    if z.godziny_oszacowane > 100 {
                        return z.przypisany.clone();
                    }
                }
            }
        }
        None
    }
    fn statystyki_statusow(&self) -> HashMap<StatusZadania, usize> {
        let mut hm: HashMap<StatusZadania, usize> = HashMap::new();
        for z in &self.zadania {
            hm.entry(z.status.clone())
                .and_modify(|i| *i += 1)
                .or_insert(1);
        }
        hm
    }
}

fn main() {
    let mut projekt = Projekt {
        id: 1,
        nazwa: "Nowy System".to_string(),
        zespol: vec![],
        zadania: vec![],
    };

    projekt.dodaj_czlonka(CzlonekZespolu {
        imie: "Anna".to_string(),
        nazwisko: "Kowalska".to_string(),
        specjalizacja: Specjalizacja::Frontend,
    });

    projekt.dodaj_czlonka(CzlonekZespolu {
        imie: "Jan".to_string(),
        nazwisko: "Nowak".to_string(),
        specjalizacja: Specjalizacja::Backend,
    });

    projekt.dodaj_zadanie(Zadanie {
        nazwa: "Zbuduj UI".to_string(),
        przypisany: None,
        status: StatusZadania::Nowe,
        godziny_oszacowane: 50,
        godziny_rzeczywiste: None,
    });

    projekt.dodaj_zadanie(Zadanie {
        nazwa: "API dla UI".to_string(),
        przypisany: None,
        status: StatusZadania::Nowe,
        godziny_oszacowane: 60,
        godziny_rzeczywiste: None,
    });

    projekt.przypisz_zadanie("Zbuduj UI", "Kowalska").unwrap();
    projekt.przypisz_zadanie("API dla UI", "Nowak").unwrap();

    projekt.zakoncz_zadanie("Zbuduj UI", 48).unwrap();
    projekt
        .odrzuc_zadanie("API dla UI", "Złe wymagania".to_string())
        .unwrap();

    let godziny = projekt.godziny_na_osobe();
    println!("Godziny na osobę: {:?}", godziny);

    let zadania_front = projekt.zadania_dla_specjalizacji(Specjalizacja::Frontend);
    println!("Zadania dla Frontendu: {:?}", zadania_front);

    println!("\n=== Raport ===");
    projekt.raport();

    if let Some(przeciazony) = projekt.przeciazony_pracownik() {
        println!("Przeciążony pracownik: {}", przeciazony);
    }

    let statystyki = projekt.statystyki_statusow();
    println!("Statystyki statusów: {:?}", statystyki);
}
