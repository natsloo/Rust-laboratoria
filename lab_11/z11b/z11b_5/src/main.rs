#[derive (PartialEq, Debug)]
enum WarunkiPogodowe {
    Slonce,
    Chmury,
    Deszcz,
    Snieg
}
#[derive (Debug)]
struct DanePogodowe {
    lokalizacja: String,
    kolejny_dzien_obserwacji: u32,
    temperatura: f32,
    warunki_pogodowe: WarunkiPogodowe
}

impl DanePogodowe {
    fn new(lokalizacja: String, kolejny_dzien_obserwacji: u32, temperatura: f32, warunki_pogodowe: WarunkiPogodowe) -> Self {
        Self {
            lokalizacja,
            kolejny_dzien_obserwacji,
            temperatura,
            warunki_pogodowe
        }
    }
}

struct DziennikPogodowy {
    dane: Vec<DanePogodowe>
}

impl DziennikPogodowy {
    fn new() -> Self {
        Self {
            dane: Vec::new()
        }
    }
    fn srednia_dla_warunkow(&self, warunki: WarunkiPogodowe) -> f32 {
        let mut suma = 0.0;
        let mut licznik = 0;
        for d in &self.dane {
            if d.warunki_pogodowe == warunki {
                suma += d.temperatura;
                licznik += 1;
            }
        }

        suma/licznik as f32
    }
    fn najczestszy_typ_pogody(&self) -> WarunkiPogodowe {
        let mut slonce = 0;
        let mut chmury = 0;
        let mut deszcz = 0;
        let mut snieg = 0;
        for d in &self.dane {
            match d.warunki_pogodowe {
                WarunkiPogodowe::Slonce => slonce += 1,
                WarunkiPogodowe::Chmury => chmury += 1,
                WarunkiPogodowe::Deszcz => deszcz += 1,
                WarunkiPogodowe::Snieg => snieg += 1
            }
        }
        let max = slonce.max(chmury).max(deszcz).max(snieg);
        match max {
            max if max == slonce => WarunkiPogodowe::Slonce,
            max if max == chmury => WarunkiPogodowe::Chmury,
            max if max == deszcz => WarunkiPogodowe::Deszcz,
            _ => WarunkiPogodowe::Snieg
        }
    }
    fn sloneczne_dni(&self) -> Vec<&DanePogodowe> {
        let mut sloneczne = Vec::new();
        for d in &self.dane {
            if d.warunki_pogodowe == WarunkiPogodowe::Slonce && d.temperatura > 30.0 {
                sloneczne.push(d);
            }
        }
        sloneczne
    }
    fn dodaj_dane(&mut self, lokalizacja: String, temperatura: f32, warunki_pogodowe: WarunkiPogodowe) {
        let mut ostatni = 0;
        for d in &self.dane {
            if d.lokalizacja == lokalizacja {
                if d.kolejny_dzien_obserwacji > ostatni {
                    ostatni = d.kolejny_dzien_obserwacji
                }
            }
        }
        self.dane.push(DanePogodowe::new(lokalizacja,ostatni+1,temperatura,warunki_pogodowe));
    }
}



fn main() {
    let mut dziennik = DziennikPogodowy::new();
    dziennik.dodaj_dane("Lublin".to_string(), 14.0, WarunkiPogodowe::Chmury);
    dziennik.dodaj_dane("Lublin".to_string(), 16.5, WarunkiPogodowe::Deszcz);
    dziennik.dodaj_dane("Lublin".to_string(), 31.2, WarunkiPogodowe::Slonce);
    dziennik.dodaj_dane("Lublin".to_string(), -2.0, WarunkiPogodowe::Snieg);

    dziennik.dodaj_dane("Krakow".to_string(), 30.1, WarunkiPogodowe::Slonce);
    dziennik.dodaj_dane("Krakow".to_string(), 18.3, WarunkiPogodowe::Chmury);
    dziennik.dodaj_dane("Krakow".to_string(), 12.9, WarunkiPogodowe::Deszcz);
    dziennik.dodaj_dane("Krakow".to_string(), -1.5, WarunkiPogodowe::Snieg);

    dziennik.dodaj_dane("Warszawa".to_string(), 29.8, WarunkiPogodowe::Slonce);
    dziennik.dodaj_dane("Warszawa".to_string(), 32.5, WarunkiPogodowe::Slonce);
    dziennik.dodaj_dane("Warszawa".to_string(), 20.0, WarunkiPogodowe::Chmury);
    dziennik.dodaj_dane("Warszawa".to_string(), 15.0, WarunkiPogodowe::Deszcz);

    dziennik.dodaj_dane("Gdansk".to_string(), 18.3, WarunkiPogodowe::Chmury);
    dziennik.dodaj_dane("Gdansk".to_string(), 17.0, WarunkiPogodowe::Chmury);
    dziennik.dodaj_dane("Gdansk".to_string(), 19.5, WarunkiPogodowe::Deszcz);
    dziennik.dodaj_dane("Gdansk".to_string(), 8.0, WarunkiPogodowe::Snieg);

    println!("Średnia temperatura dla Słońca: {:.1}", dziennik.srednia_dla_warunkow(WarunkiPogodowe::Slonce));
    println!("Najczęstsze warunki: {:?}", dziennik.najczestszy_typ_pogody());
    println!("Słoneczne dni powyżej 30°C:");
    for wpis in dziennik.sloneczne_dni() {
        println!("{:?}", wpis);
    }
}
