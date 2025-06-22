use crate::karta::Karta;
use crate::wartosc::Wartosc;

#[derive(Clone)]
pub struct Reka {
    karty: Vec<Karta>
}

impl Reka {
    pub fn new() -> Self {
        Reka {
            karty: Vec::new()
        }
    }
    pub fn dobierz_karte(&mut self, k: Option<Karta>){
        if k.is_some(){
            let karta = k.unwrap();
            println!("Twoja karta to: {}",karta.opis());
            self.karty.push(karta);
        }
        else {
            println!("Nie mozna dobrac karty!\n");
        }
    }
    pub fn ile_kart_w_rece(&self) -> usize {
        self.karty.len()
    }
    pub fn suma_punktow(&self) -> (u8,String) {
        let mut suma = 0;
        let mut komunikat = String::new();
        if self.karty.len() >= 2 {
            let pierwsza = self.karty[0];
            let druga = self.karty[1];
            if matches!(pierwsza.wartosc(),Wartosc::As) && matches!(druga.wartosc(),Wartosc::As) {
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