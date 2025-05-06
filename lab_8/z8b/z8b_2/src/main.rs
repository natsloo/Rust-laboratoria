#[derive(Debug, PartialEq, Clone)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogramy
}
#[derive(Debug, PartialEq, Clone)]
enum Warunki {
    Zamrazarka,
    Chlodziarka,
    Normalne
}
#[derive(Debug, PartialEq, Clone)]
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

#[derive (Debug)]
struct Zamowienie {
    lista_towarow: Vec<(Towar,f64)>
}
impl Zamowienie{
    fn new() -> Zamowienie{
        Zamowienie{
            lista_towarow: Vec::new()
        }
    }
    fn sumaryczna_waga_zamowienia(&self) -> f64 {
        let mut suma = 0.0;
        for (t,i) in &self.lista_towarow {
            suma += t.waga_w_kg * i;
        }
        suma
    }
    fn sumaryczna_waga_wg_warunkow(&self, warunki: Warunki) -> f64 {
        let mut suma = 0.0;
        for (t,i) in &self.lista_towarow {
            if t.warunki == warunki {
                suma += t.waga_w_kg * i;
            }
        }
        suma
    }
    fn dodaj_towar(&mut self, towar: Towar, ilosc: f64) {
        if ilosc < 0.0 {
            println!("Ilość musi być dodatnia!");
            return;
        }
        if Jednostka::Sztuki == towar.jednostka {
            if ilosc.fract() != 0.0 {
                println!("Sztuki muszą mieć całkowitą ilość!");
                return;
            }
        }
        for (t, i) in &mut self.lista_towarow {
            if *t == towar {
                *i += ilosc;
                return;
            }
        }
        self.lista_towarow.push((towar,ilosc));
    }
}


fn main() {
    let mut zamowienie = Zamowienie::new();

    let mleko = Towar::new("Mleko".to_string(), Jednostka::Litry, 1.0, Warunki::Chlodziarka).unwrap();
    let lody = Towar::new("Lody".to_string(), Jednostka::Kilogramy, 1.0, Warunki::Zamrazarka).unwrap();
    let jajka = Towar::new("Jajka".to_string(), Jednostka::Sztuki, 0.06, Warunki::Normalne).unwrap();
    let jogurt = Towar::new("Jogurt".to_string(), Jednostka::Sztuki, 0.4, Warunki::Chlodziarka).unwrap();

    // Dodawanie poprawnych towarów
    zamowienie.dodaj_towar(mleko.clone(), 3.0);
    zamowienie.dodaj_towar(jogurt.clone(), 10.0);
    zamowienie.dodaj_towar(lody.clone(), 2.0);
    zamowienie.dodaj_towar(jajka.clone(), 12.0);

    // Próba dodania z błędną ilością sztuk (niecałkowita)
    zamowienie.dodaj_towar(jajka.clone(), 2.5); // powinien wypisać błąd

    // Próba dodania ujemnej ilości
    zamowienie.dodaj_towar(mleko.clone(), -1.0); // powinien wypisać błąd

    println!("Zamówienie: {:?}", zamowienie);
    println!(
        "Sumaryczna waga: {} kg",
        zamowienie.sumaryczna_waga_zamowienia()
    );
    println!(
        "Waga chłodzonych produktów: {} kg",
        zamowienie.sumaryczna_waga_wg_warunkow(Warunki::Chlodziarka)
    );
}
