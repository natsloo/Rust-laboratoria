fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() {
        return None;
    }

    let mut result = String::new();

    for c in z.chars(){
        match c {
            '0' => result.push_str("000"),
            '1' => result.push_str("001"),
            '2' => result.push_str("010"),
            '3' => result.push_str("011"),
            '4' => result.push_str("100"),
            '5' => result.push_str("101"),
            '6' => result.push_str("110"),
            '7' => result.push_str("111"),
            _ => return None,
        }
    }

    while result.starts_with('0') && result.len() > 1  {
        result.remove(0);
    }
    Some(result)
}

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

fn wartosc_syst8(z: &str) -> Option<u8> {
    let a = zamien_syst8_na_syst2(z);
    if a.is_none() {
        return None
    }
    else {
        let a = a.unwrap();
        wartosc_syst2(&a)
    }
}

fn wartosc_syst8_v2(z: &str) -> Option<u8> {
    wartosc_syst2(&zamien_syst8_na_syst2(z)?)
}

fn main() {
    println!("{:?}", wartosc_syst8_v2("74"));
}
