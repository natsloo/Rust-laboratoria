use z12a_3::urna::Urna;
use z12a_3::randgen::RandGen;

#[derive (PartialEq, Clone)]
enum Moneta {
    Orzel,
    Reszka
}

fn testy_moneta(){
    let mut urna = Urna::<Moneta>::new(RandGen::new(123));

    let a: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);

    println!("{:?}", urna.rozmiar() == 2);
    let y: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let x: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let z: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", z.is_some());
    println!("{:?}", urna.rozmiar() == 0);
}

fn main(){
    testy_moneta();
}