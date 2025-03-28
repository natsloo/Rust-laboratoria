fn wizytowka(imie: &str, nazwisko:&str) -> String {
    let mut result:String = String::new();
    let im= imie.chars().nth(0).unwrap().to_uppercase().to_string();
    let naz = nazwisko.chars().as_str().to_lowercase();
    let n = naz.chars().nth(0).unwrap().to_uppercase().to_string();
    result.push_str(im.as_str());
    result.push_str(". ");
    result.push_str(n.as_str());
    for a in 1..naz.len() {
        let c = naz.chars().nth(a).unwrap().to_lowercase().to_string();
        //println!("c {}", c);
        result.push_str(c.as_str());
    }
    result
}

fn main() {
    let imie = "jan";
    let nazwisko = "KOWALSKI";
    println!("{}", wizytowka(imie,nazwisko));
}
