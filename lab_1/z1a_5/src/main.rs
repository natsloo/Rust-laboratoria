fn main() {
    let h1 = 07;
    let m1 = 16;
    let s1 = 50;

    let h2 = 06;
    let m2 = 59;
    let s2 = 02;

    let sekundy1 = s1 + m1 * 60 + h1 * 3600;
    let sekundy2 = s2 + m2 * 60 + h2 * 3600;

    let roznica;

    if sekundy1 >= sekundy2 {
        roznica = sekundy1 - sekundy2;
    }
    else {
        roznica = sekundy2 - sekundy1
    }

    let h3 = roznica / 3600;
    let m2 = (roznica % 3600) / 60; //(roznica % 3600) - pozostałe sekundy - reszta z dzielenia
    let s3 = roznica % 60;

    println!("Różnica wynosi {h3}:{m2}:{s3}.")

}
