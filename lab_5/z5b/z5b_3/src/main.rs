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

fn main() {
    match wartosc_cyfry_rzymskiej('-') {
        Ok(l) => println!("{}",l),
        Err(e) => println!("{}",e)
    }
}
