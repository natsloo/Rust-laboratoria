#[derive(Copy, Clone)]
pub enum Kolor {
    Karo,               // romb                         czerwony
    Kier,               // serce                        czerwone
    Pik,                // odwrocone serce na nozce     czarne
    Trefl               // konczynka na nozce           czarna
}

impl Kolor {
    pub fn kolor_as_string(&self) -> String {
        match &self {
            Kolor::Karo => String::from("♢ Karo"),
            Kolor::Kier => String::from("♡ Kier"),
            Kolor::Pik => String::from("♠ Pik"),
            Kolor::Trefl => String::from("♣ Trefl")
        }
    }
}