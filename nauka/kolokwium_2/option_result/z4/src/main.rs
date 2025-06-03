fn wizytowka(imie: Option<String>, nazwisko: Option<String>) -> String {
    let mut i = String::new();
    let mut n = String::new();
    if imie.is_none(){
        i = String::from("Jan");
    }
    else {
        i = imie.unwrap();
    }
    if nazwisko.is_none(){
        n = String::from("Kowalski");
    }
    else {
        n = nazwisko.unwrap();
    }
    i + &String::from(" ") + &n
}

fn main() {
    println!("{}", wizytowka(None,None));
    println!("{}", wizytowka(None,Some("Nowak".to_string())));
    println!("{}", wizytowka(Some("Adam".to_string()),None));
    println!("{}", wizytowka(Some("Adam".to_string()),Some("Nowak".to_string())));
}
