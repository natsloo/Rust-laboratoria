use crate::karta::Karta;
use crate::kolor::Kolor;
use crate::wartosc::Wartosc;
use crate::randgen::RandGen;
pub struct Talia {
    talia: Vec<Karta>
}

impl Talia {
    pub fn new() -> Self {
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
    pub fn ilosc_kart(&self) -> usize {
        self.talia.len()
    }
    pub fn dobierz(&mut self) -> Option<Karta> {
        self.talia.pop()
    }
    pub fn zamien_elementy(&mut self, i: usize, j: usize) {
        let pom = self.talia[i];
        self.talia[i] = self.talia[j];
        self.talia[j] = pom;
    }
    pub fn potasuj(&mut self, rg: &RandGen,  seed: &mut i128){
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