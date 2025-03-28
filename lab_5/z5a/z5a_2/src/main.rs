fn wartosc_syst2(z: &str) -> Option<u8> {

    let mut z = String::from(z);

    while z.starts_with('0') && z.len() > 1  {
        z.remove(0);
    }

    if z.len() > 8 || z.len() < 1 {
        return None;
    }
    let mut result:u8 = 0;
    for (i, c) in z.chars().rev().enumerate(){
        if c == '1' {
            result += 2_u8.pow(i as u32);
        }
        else if c != '0' {
            return None;
        }
    }
    Some(result)

}

fn main() {
    match wartosc_syst2("") {
        Some(dec) => println!("{}", dec),
        None => println!("Błędny ciąg!"),
    }
    println!("{:?}", wartosc_syst2("00000001"));
    println!("{:?}", wartosc_syst2(""));
    println!("{:?}", wartosc_syst2("11111111"));
    println!("{:?}", wartosc_syst2("01010100"));
    println!("{:?}", wartosc_syst2("000010000001"));
    println!("{:?}", wartosc_syst2("100000001"));

}
