use crate::kolor::Kolor;
use crate::wartosc::Wartosc;

#[derive(Clone,Copy)]
pub struct Karta {
    kolor: Kolor,
    wartosc: Wartosc
}

impl Karta {
    pub fn new(kolor: Kolor, wartosc: Wartosc) -> Self {
        Karta {
            kolor,
            wartosc
        }
    }
    pub fn kolor_string(&self) -> String {
        self.kolor.kolor_as_string()
    }
    pub fn wartosc_u8(&self) -> u8 {
        self.wartosc.wartosc_as_u8()
    }
    pub fn wartosc_string(&self) -> String {
        self.wartosc.wartosc_as_string()
    }
    pub fn opis(&self) -> String {
        let kolor = self.kolor_string();
        let spacja = String::from(" ");
        let wartosc = self.wartosc_string();
        wartosc + &spacja + &kolor
    }
    pub fn wartosc(&self) -> Wartosc{
        self.wartosc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kolory_wartosci() {
        let k = Karta::new(Kolor::Karo,Wartosc::Dwojka);
        assert_eq!(k.kolor_string(),"♢ Karo");
        let k2 = Karta::new(Kolor::Kier,Wartosc::As);
        assert_eq!(k2.wartosc_u8(),11);
        let k3 = Karta::new(Kolor::Pik,Wartosc::Walet);
        assert_eq!(k3.wartosc_string(), "Walet");
    }
}