#[derive(Debug)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogramy
}
#[derive(Debug)]
enum Warunki {
    Zamrazarka,
    Chlodziarka,
    Normalne
}
#[derive(Debug)]
struct Towar{
    opis: String,
    jednostka: Jednostka,
    waga_w_kg: f64,
    warunki: Warunki
}
impl Towar {
    fn new_kg(opis: String, warunki: Warunki) -> Self {
        Towar {
            opis,
            jednostka:Jednostka::Kilogramy,
            waga_w_kg: 1.0,
            warunki
        }
    }
    fn new(opis: String, jednostka: Jednostka, waga_w_kg: f64, warunki: Warunki) -> Option<Self> {
        if waga_w_kg < 0.0 {
            return None
        }
        else {
            if let Jednostka::Kilogramy = jednostka {
                if waga_w_kg != 1.0 {
                    return None
                }
            }
            Some(Towar {
                opis,
                jednostka,
                waga_w_kg,
                warunki
            })
        }
    }
}


fn main() {
    // Towar z jednostką Kilogramy (poprawny)
    let ziemniaki = Towar::new("Ziemniaki".to_string(), Jednostka::Kilogramy, 1.0, Warunki::Normalne);
    match ziemniaki {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("Błąd przy tworzeniu ziemniaków\n"),
    }

    // Towar z jednostką Kilogramy (błędna waga)
    let maka = Towar::new("Mąka".to_string(), Jednostka::Kilogramy, 0.75, Warunki::Normalne);
    match maka {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("❌ Nie udało się utworzyć mąki (waga ≠ 1.0)\n"),
    }

    // Towar z jednostką Litry (poprawny)
    let mleko = Towar::new("Mleko 2%".to_string(), Jednostka::Litry, 1.05, Warunki::Chlodziarka);
    match mleko {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("Błąd przy tworzeniu mleka\n"),
    }

    // Towar z jednostką Sztuki (ujemna waga)
    let puszka = Towar::new("Puszka kukurydzy".to_string(), Jednostka::Sztuki, -0.25, Warunki::Normalne);
    match puszka {
        Some(t) => println!("Poprawnie utworzono:\n{:?}\n", t),
        None => println!("❌ Nie udało się utworzyć puszki (ujemna waga)\n"),
    }

    // Towar przez new_kg (skrócony konstruktor)
    let jablka = Towar::new_kg("Jabłka ligol".to_string(), Warunki::Normalne);
    println!("Towar utworzony przez new_kg:\n{:?}\n", jablka);

}
