fn is_perfect_number(a: u64) -> bool {              // wszystkie dzielniki, nie tylko pierwsze
    if a <= 1 {
        return false;
    }

    let mut suma:u64 = 1;                           // 1 zawsze jest dzielnikiem

    for i in 2..=((a as f64).sqrt() as u64) {  // jedziemy od 2 do pierwiastka z a
        if a % i == 0 {
            suma += i;
            let dzielnik = a / i;              // znajdujemy parę do dzielnika
            if dzielnik != i {                      // sprawdzenie potrzebne, aby nie dodawać tego
                suma += dzielnik;                   // samego dzielnika dwa razy, gdy liczba a
            }                                       // jest kwadratem liczby całkowitej
        }                                           // 36 = 6 * 6, do sumy trzeba dodać tylko jedną szóstkę
    }
    //println!("suma: {}", suma);
    // if a == suma {
    //     return true;
    // }
    // false

    suma == a                                       // wynik tego wyrażenia to bool, nie trzeba ifa
}

fn main() {
    println!("{}", is_perfect_number(137438691328));
    println!("{}", is_perfect_number(28));
    println!("{}", is_perfect_number(8128));
    println!("{}", is_perfect_number(8129));
}
