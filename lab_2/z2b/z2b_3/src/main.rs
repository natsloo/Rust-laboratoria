fn is_armstrong_number(a: u128) -> bool {
    let mut liczba = a;
    let liczba_cyfr = a.to_string().len();
    let mut suma = 0;
    while liczba > 0 {
        let ostatnia = liczba % 10;
        suma += ostatnia.pow(liczba_cyfr as u32);
        liczba /= 10;
    }
    suma == a
}


fn main() {
    println!("{}", is_armstrong_number(9926315));
}
