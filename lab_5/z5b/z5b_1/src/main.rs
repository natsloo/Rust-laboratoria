fn wartosc_cyfry(c: char) -> Result<u8, String> {
    let a = c.to_digit(10);
    if a.is_none() {
        Err("Znak nie jest liczbą!".to_string())
    }
    else {
        Ok(a.unwrap() as u8)
    }
}

fn wartosc_cyfry_v2(c: char) -> Result<u8, String> {
    c.to_digit(10).map(|l| l as u8).ok_or("Znak nie jest liczbą!".to_string())
}

fn main() {
    match wartosc_cyfry('1') {
        Ok(l) => println!("{}", l),
        Err(e) => println!("{}", e),
    }
}
