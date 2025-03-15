fn integer_factorization(mut a: i64) {
    if a <= 1 {
        println!("Liczby {} nie da się rozłożyć na czynniki.", a)
    }

    print!("Rozkład {} na czynniki pierwsze: ", a);

    while a % 2 == 0 {           // 2 - jedyna parzysta pierwsza, więc pozbywamy się jej (i jej wielokrotności) na początku
        print!("{} ", 2);
        a /= 2
    }

    let mut i = 3;
    while i * i <= a {           // to działa tak samo jak i <= sqrt(a): i <= od pierwiastka albo i^2 <= od całej liczby
        while a % i == 0 {       // dopóki liczba się dzieli przez i
            print!("{} ", i);    // wydrukuj i
            a /= i;              // podziel przez i
        }
        i += 2                   // przejdź do następnej liczby nieparzystej
    }

    if a > 1 {                   // jeśli została jakaś liczba > 1, to jest ona pierwsza, więc ją wypisujemy
        print!("{}", a);         // ten if obsluguje też od razu przypadek, gdy na wejściu liczba jest pierwsza
    }
}

fn main() {
    integer_factorization(738);
}
