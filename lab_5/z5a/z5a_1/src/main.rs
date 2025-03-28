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

fn main() {
    println!("{}", zamien_syst8_na_syst2("741").unwrap_or("Nie da się zamienić!".to_string()));
}
