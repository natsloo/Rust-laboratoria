#[derive(Copy, Clone)]
pub enum Wartosc {
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
    pub fn wartosc_as_u8(&self) -> u8 {
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
    pub fn wartosc_as_string(&self) -> String {
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