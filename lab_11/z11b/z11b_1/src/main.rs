#[derive(PartialEq, Debug)]
enum Gatunek {
    SciFi,
    Reportaz,
    Fantastyka,
    Popularnonaukowe,
    Romans,
}

#[derive(Debug)]
struct Ksiazka {
    tytul: String,
    autor: String,
    liczba_stron: u16,
    gatunek: Gatunek
}

impl Ksiazka {
    fn new(tytul: String, autor: String, liczba_stron: u16, gatunek: Gatunek) -> Self {
        Ksiazka {
            tytul,
            autor,
            liczba_stron,
            gatunek
        }
    }
}

fn filtruj_po_gatunku(ksiazki: Vec<Ksiazka>, gatunek: Gatunek) -> Vec<Ksiazka> {
    let mut wybrane = Vec::new();
    for k in ksiazki {
        if k.gatunek == gatunek {
            wybrane.push(k);
        }
    }
    wybrane
}


fn main() {
    let mut ksiazki = Vec::new();
    ksiazki.push(Ksiazka::new("Zbrodnia i kara".to_string(), "Fiodor Dostojewski".to_string(), 670, Gatunek::SciFi));
    ksiazki.push(Ksiazka::new("Duma i uprzedzenie".to_string(), "Jane Austen".to_string(), 432, Gatunek::Romans));
    ksiazki.push(Ksiazka::new("Lśnienie".to_string(), "Stephen King".to_string(), 512, Gatunek::Fantastyka));
    ksiazki.push(Ksiazka::new("Mikołajek".to_string(), "René Goscinny".to_string(), 150, Gatunek::Reportaz));
    ksiazki.push(Ksiazka::new("Mały książę".to_string(), "Antoine de Saint-Exupéry".to_string(), 96, Gatunek::Popularnonaukowe));
    ksiazki.push(Ksiazka::new("Folwark zwierzęcy".to_string(), "George Orwell".to_string(), 144, Gatunek::SciFi));
    ksiazki.push(Ksiazka::new("Stary człowiek i morze".to_string(), "Ernest Hemingway".to_string(), 127, Gatunek::Fantastyka));
    ksiazki.push(Ksiazka::new("Cierpienia młodego Wertera".to_string(), "Johann Wolfgang von Goethe".to_string(), 224, Gatunek::Romans));
    println!("{:?}", ksiazki);
    println!();
    println!("{:?}", filtruj_po_gatunku(ksiazki, Gatunek::Romans));
}
