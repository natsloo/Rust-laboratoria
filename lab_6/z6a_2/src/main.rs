fn shorter_than_four(test_strings: Vec<&str>) -> Vec<String> {
    let result = test_strings.iter().filter(|x| x.len() < 4).map(|x| x.to_string()).collect();
    result
}

fn no_a_or_A(test_strings: Vec<&str>) -> Vec<String> {
    let result = test_strings.iter().filter(|x| !x.contains('A') && !x.contains('a')).map(|x| x.to_string()).collect();
    result
}

fn contains_numbers(test_strings: Vec<&str>) -> Vec<String> {
    let result = test_strings.iter().filter(|x| x.chars().any(|c| c.is_digit(10))).map(|x| x.to_string()).collect();
    result
}

fn reversed_strings(test_strings: Vec<&str>) -> Vec<String> {
    let result = test_strings.into_iter().map(|x| x.chars().rev().collect()).collect();
    result
}

fn has_double_letters(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
    // s.chars() tworzy iterator po znakach w napisie
    // s.chars().skip(1) tworzy drugi iterator, pomijając 1 znak
    // zip() łączy oba te iteratory i dla danej pary liter wyciągniętej z obu powstaje krotka
    // krotka ta jest potem używana do sprawdzenia, czy któraś z liter się powtarza
}
fn doubled_letter(test_strings: Vec<&str>) -> Vec<String> {
    let result = test_strings
        .into_iter().filter(|x| has_double_letters(x))
        .map(|x| x.to_string()).collect();
    result
}

fn main() {
    let test_strings:Vec<&str> = vec![
        "kot", "pies", "dom", "Ala", "zamek", "król", "robot", "1234", "test1",
        "inny", "pizza", "brutto", "lekki", "dzienny", "programowanie", "Rust",
        "wow", "gamma", "delta", "epsilon", "hello123", "abcd", "xyz", "foo", "bar",
    ];
    println!("shorter_than_four: {:?}\n", shorter_than_four(test_strings.clone()));
    println!("no_a_or_A: {:?}\n", no_a_or_A(test_strings.clone()));
    println!("contains_numbers: {:?}\n", contains_numbers(test_strings.clone()));
    println!("reversed_strings: {:?}\n", reversed_strings(test_strings.clone()));
    println!("doubled_letter: {:?}\n", doubled_letter(test_strings.clone()));

}
