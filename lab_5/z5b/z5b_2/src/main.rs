fn wartosc_cyfry(c: char) -> Result<u8, String> {
    let a = c.to_digit(10);
    if a.is_none() {
        Err("Znak nie jest liczbą!".to_string())
    }
    else {
        Ok(a.unwrap() as u8)
    }
}

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    let a: Vec<char> = a.chars().rev().collect();
    let b: Vec<char> = b.chars().rev().collect();

    let max_len = std::cmp::max(a.len(),b.len());

    let mut przeniesienie = 0;
    let mut wynik = String::new();

    for c in 0..max_len {
        let digit_a;
        let digit_b;
        if c < a.len() {
            digit_a = wartosc_cyfry(a[c])?;
        }
        else {
            digit_a = 0;
        }
        if c < b.len(){
            digit_b = wartosc_cyfry(b[c])?
        }
        else {
            digit_b = 0;
        }
        let suma = digit_a + digit_b + przeniesienie;
        let jednosci = suma % 10;
        wynik.push(char::from_digit(jednosci as u32, 10).unwrap());
        przeniesienie = suma / 10;
    }
    if przeniesienie > 0 {
        wynik.push(char::from_digit(przeniesienie as u32, 10).unwrap());
    }

    Ok(wynik.chars().rev().collect())
}


fn main() {
    match dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298") {
        Ok(l) => println!("{}", l),
        Err(e) => println!("{}", e),
    }
}
