use std::io;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
#[derive(Copy, Clone)]
enum Kolor {
    Karo,               // romb                         czerwony
    Kier,               // serce                        czerwone
    Pik,                // odwrocone serce na nozce     czarne
    Trefl               // konczynka na nozce           czarna
}

impl Kolor {
    fn kolor_as_string(&self) -> String {
        match &self {
            Kolor::Karo => String::from("♢ Karo"),
            Kolor::Kier => String::from("♡ Kier"),
            Kolor::Pik => String::from("♠ Pik"),
            Kolor::Trefl => String::from("♣ Trefl")
        }
    }
}
#[derive(Copy, Clone)]
enum Wartosc {
    Dwojka,
    Trojka,
    Czworka,
    Piatka,
    Szostka,
    Siodemka,
    Osemka,
    Dziewiatka,
    Dziesiatka,
    Walet,
    Dama,
    Krol,
    As
}

impl Wartosc {
    fn wartosc_as_u8(&self) -> u8 {
        match &self {
            Wartosc::Dwojka => 2,
            Wartosc::Trojka => 3,
            Wartosc::Czworka => 4,
            Wartosc::Piatka => 5,
            Wartosc::Szostka => 6,
            Wartosc::Siodemka => 7,
            Wartosc::Osemka => 8,
            Wartosc::Dziewiatka => 9,
            Wartosc::Dziesiatka => 10,
            Wartosc::Walet => 2,
            Wartosc::Dama => 3,
            Wartosc::Krol => 4,
            Wartosc::As => 11
        }
    }
    fn wartosc_as_string(&self) -> String {
        match &self {
            Wartosc::Dwojka => String::from("Dwojka"),
            Wartosc::Trojka => String::from("Trojka"),
            Wartosc::Czworka => String::from("Czworka"),
            Wartosc::Piatka => String::from("Piatka"),
            Wartosc::Szostka => String::from("Szostka"),
            Wartosc::Siodemka => String::from("Siodemka"),
            Wartosc::Osemka => String::from("Osemka"),
            Wartosc::Dziewiatka => String::from("Dziewiatka"),
            Wartosc::Dziesiatka => String::from("Dziesiatka"),
            Wartosc::Walet => String::from("Walet"),
            Wartosc::Dama => String::from("Dama"),
            Wartosc::Krol => String::from("Krol"),
            Wartosc::As => String::from("As")
        }
    }
}
#[derive(Clone,Copy)]
struct Karta {
    kolor: Kolor,
    wartosc: Wartosc
}

impl Karta {
    fn new(kolor: Kolor, wartosc: Wartosc) -> Self {
        Karta {
            kolor,
            wartosc
        }
    }
    fn kolor_string(&self) -> String {
        self.kolor.kolor_as_string()
    }
    fn wartosc_u8(&self) -> u8 {
        self.wartosc.wartosc_as_u8()
    }
    fn wartosc_string(&self) -> String {
        self.wartosc.wartosc_as_string()
    }
    fn opis(&self) -> String {
        let kolor = self.kolor_string();
        let spacja = String::from(" ");
        let wartosc = self.wartosc_string();
        wartosc + &spacja + &kolor
    }
}

struct Talia {
    talia: Vec<Karta>
}

impl Talia {
    fn new() -> Self {
        let mut talia = Vec::new();
        let kolory = [Kolor::Karo, Kolor::Kier, Kolor::Pik, Kolor::Trefl];
        let wartosci = [Wartosc::Dwojka, Wartosc::Trojka, Wartosc::Czworka,
        Wartosc::Piatka, Wartosc::Szostka, Wartosc::Siodemka, Wartosc::Osemka,
        Wartosc::Dziewiatka, Wartosc::Dziesiatka, Wartosc::Walet, Wartosc::Dama,
        Wartosc::Krol, Wartosc::As];
        for kolor in kolory {
            for wartosc in wartosci {
                let k = Karta::new(kolor, wartosc);
                talia.push(k);
            }
        }
        Talia {
            talia
        }
    }
    fn ilosc_kart(&self) -> usize {
        self.talia.len()
    }
    fn dobierz(&mut self) -> Option<Karta> {
        self.talia.pop()
    }
    fn zamien_elementy(&mut self, i: usize, j: usize) {
        let pom = self.talia[i];
        self.talia[i] = self.talia[j];
        self.talia[j] = pom;
    }
    fn potasuj(&mut self, rg: &RandGen,  seed: &mut i128){
        let r = rg.rand(seed,0,self.talia.len() as i128) as usize;
        rg.change_seed(&mut (*seed + 8));
        let mut counter = 0;
        while counter < r {
            let i = rg.rand(seed, 0, (self.talia.len() - 1) as i128) as usize;
            let j = rg.rand(seed, 0, (self.talia.len() - 1) as i128) as usize;
            self.zamien_elementy(i, j);
            counter += 1;
        }
    }
}

struct RandGen{}

impl RandGen {
    fn new() -> Self {
        RandGen{}
    }
    fn change_seed(&self, seed: &mut i128) {
        let a = 22695477;
        *seed = (a * *seed + 1) % 2_i128.pow(31);
    }

    fn rand(&self, seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
        self.change_seed(seed);
        *seed % (max_rand - min_rand + 1) + min_rand
    }
}

struct Reka {
    karty: Vec<Karta>
}

impl Reka {
    fn new() -> Self {
        Reka {
            karty: Vec::new()
        }
    }
    fn dobierz_karte(&mut self, k: Option<Karta>){
        if k.is_some(){
            let karta = k.unwrap();
            println!("Twoja karta to: {}",karta.opis());
            self.karty.push(karta);
        }
        else {
            println!("Nie mozna dobrac karty!\n");
        }
    }
    fn ile_kart_w_rece(&self) -> usize {
        self.karty.len()
    }
    fn suma_punktow(&self) -> (u8,String) {
        let mut suma = 0;
        let mut komunikat = String::new();
        if self.karty.len() >= 2 {
            let pierwsza = self.karty[0];
            let druga = self.karty[1];
            if matches!(pierwsza.wartosc,Wartosc::As) && matches!(druga.wartosc,Wartosc::As) {
                komunikat = "Perskie oczko! Dwa asy dają 21 punktow i natychmiastowe zwyciestwo.".to_string();
                return (21, komunikat)
            }
        }
        for k in &self.karty {
            suma += k.wartosc_u8();
        }
        komunikat = format!("Suma punktow w Twojej rece wynosi {}",suma);
        (suma, komunikat)
    }
}

struct Gracz {
    reka: Reka
}

impl Gracz {
    fn new() -> Self {
        Gracz {
            reka: Reka::new()
        }
    }
}

struct Gra {
    talia: Talia,
    gracz: Gracz,
    rg: RandGen,

}

impl Gra {
    fn new() -> Self {
        Gra {
            talia: Talia::new(),
            gracz: Gracz::new(),
            rg: RandGen::new(),
        }

    }
    fn graj(&mut self,seed: &mut i128) {
        self.talia.potasuj(&self.rg,seed);
        let mut suma;
        let mut komunikat;
        loop {
            let mut input = String::new();
            println!("\nWprowadz znak +, jesli chcesz dobrac karte lub -, jesli chcesz zakonczyc gre!");
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input == "+" {
                let k = self.talia.dobierz();
                self.gracz.reka.dobierz_karte(k);
                (suma,komunikat) = self.gracz.reka.suma_punktow();
                println!("{}",komunikat);
                if suma == 21 {
                    println!("Gratulacje, wygrales!");
                    break;
                }
                if suma > 21 {
                    println!("Przegrana!");
                    break;
                }
            }
            if input == "-" {
                (suma,komunikat) = self.gracz.reka.suma_punktow();
                println!("{}",komunikat);
                println!("Byles o {} punktow od wygranej.", 21 - suma);
                break;
            }
        }
    }
}

fn startowy_seed() -> i128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("blad generacji seedu startowego!").as_millis() as i128
}

fn main() {
    let mut g = Gra::new();
    g.graj(&mut startowy_seed())
}
