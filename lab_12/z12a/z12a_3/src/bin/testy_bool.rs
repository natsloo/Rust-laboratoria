use z12a_3::urna::Urna;
use z12a_3::randgen::RandGen;

fn testy_bool(){
    let mut urna = Urna::<bool>::new(RandGen::new(123));

    let a: Option<bool> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());

    urna.doloz(true);
    urna.doloz(false);

    println!("{:?}", urna.rozmiar() == 2);
    let y: Option<bool> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let x: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    println!("{:?}", x != y);
    let z: Option<bool> = urna.losuj_z_us();
    println!("{:?}", z.is_some());
    println!("{:?}", urna.rozmiar() == 0);
}

fn main(){
    testy_bool();
}