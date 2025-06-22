use crate::talia::Talia;
use crate::gracz::Gracz;
use crate::randgen::RandGen;
use std::io;
pub struct Gra {
    talia: Talia,
    gracz: Gracz,
    rg: RandGen,

}

impl Gra {
    pub fn new() -> Self {
        Gra {
            talia: Talia::new(),
            gracz: Gracz::new(),
            rg: RandGen::new(),
        }

    }
    pub fn graj(&mut self,seed: &mut i128) {
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
                self.gracz.reka().dobierz_karte(k);
                (suma,komunikat) = self.gracz.reka().suma_punktow();
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
                (suma,komunikat) = self.gracz.reka().suma_punktow();
                println!("{}",komunikat);
                println!("Byles o {} punktow od wygranej.", 21 - suma);
                break;
            }
        }
    }
}