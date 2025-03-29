fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    let symbols = [('I', 1), ('V',5), ('X', 10),
        ('L', 50), ('C', 100), ('D', 500), ('M', 1000)];
    for (string, value) in symbols {
        if c == string {
            return Ok(value)
        }
    }
    Err("Znak nie jest cyfrą rzymską!".to_string())
}

fn rzymskie(napis: &str) -> Result<u128, String> {
    let znaki:Vec<char> = napis.chars().rev().collect();

    let mut poprzedni = 0;
    let mut wynik = 0;
    let mut licznik = 1;

    for (i,&c) in znaki.iter().enumerate() {
       let w = wartosc_cyfry_rzymskiej(c)? as u128;

        if i > 0 && w == poprzedni {
            licznik += 1;

            if c == 'V' || c == 'L' || c == 'D' {
                return Err(format!("Znak {} nie może się powtarzać!", c));
            }

            if licznik > 3 {
                return Err(format!("Za dużo powtórzeń znaku {}!",c));
            }
        }
        else {
            licznik = 1;
        }

        if w >= poprzedni {
            wynik += w;
        }
        else {
            if poprzedni > 10 * w {
                return Err("Niewłaściwa kolejność cyfr!".to_string())
            }
            wynik -= w;
        }
        poprzedni = w;
    }
    Ok(wynik)
}

fn main() {
    match rzymskie("MMMCDL") {
        Ok(l) => println!("{}", l),
        Err(e) => println!("{}", e),
    }
}
