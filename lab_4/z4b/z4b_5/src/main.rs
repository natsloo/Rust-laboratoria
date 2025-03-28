fn dodaj_pisemnie(a: String, b: String) -> String {
    let x: Vec<char> = a.chars().rev().collect();
    let y: Vec<char> = b.chars().rev().collect();
    let mut przeniesienie = 0;
    let mut wynik = String::new();

    let max_len;
    if x.len() > y.len() {
        max_len = x.len()
    } else {
        max_len = y.len();
    }

    for c in 0..max_len {
        let digit_x;
        let digit_y;
        if c < x.len() {
            digit_x = x[c].to_digit(10).unwrap();
        }
        else {
            digit_x = 0;
        }

        if c < y.len() {
            digit_y = y[c].to_digit(10).unwrap();
        }
        else {
            digit_y = 0;
        }

        let suma = digit_x + digit_y + przeniesienie;
        let char_sum = suma % 10;
        wynik.push(char::from_digit(char_sum, 10).unwrap());
        przeniesienie = suma / 10;

    }

    if przeniesienie > 0 {
        wynik.push(char::from_digit(przeniesienie, 10).unwrap());
    }

    wynik.chars().rev().collect()
}

fn main() {
    let liczba1 = "5924729874298749827418582".to_string();
    let liczba2 = "678289362947209420974029".to_string();
    let suma = dodaj_pisemnie(liczba1.clone(), liczba2.clone());
    println!("{} + {} = {}", liczba1, liczba2, suma);
}
