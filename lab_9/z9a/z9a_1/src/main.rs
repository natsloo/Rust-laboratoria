#[derive(PartialOrd, PartialEq)]
enum Kolory{
    Trefl,
    Karo,
    Kier,
    Pik,
}

fn main() {
    println!("{}", Kolory::Pik < Kolory::Trefl);
    println!("{}", Kolory::Kier < Kolory::Pik);
    println!("{}", Kolory::Trefl < Kolory::Karo);
    println!("{}", Kolory::Pik > Kolory::Trefl);
    println!("{}", Kolory::Karo == Kolory::Karo);
    println!("{}", Kolory::Karo <= Kolory::Karo);
    println!("{}", Kolory::Karo >= Kolory::Kier);
}
