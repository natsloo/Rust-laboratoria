fn liczba_wystapien(napis: &str, znak: char) -> usize {
    let mut l = 0;
    for (_,v) in napis.chars().enumerate() {
        if znak == v {
            l += 1;
        }
    }
    l
}

fn liczba_wystapien2(napis: &str, znak: char) -> usize {
    napis.chars().filter(|x| x == &znak).count()
}

fn liczba_wystapien3(napis: &str, znak: char) -> usize {
    napis.matches(znak).count()
}

fn main() {
    println!("{}",liczba_wystapien("Konstantynopolitańczykowianeczka", 'k'));
    println!("{}",liczba_wystapien2("Lublin", 'k'));
    println!("{}",liczba_wystapien3("Konstantynopolitańczykowianeczka", 'a'));
    println!("{}",liczba_wystapien("Lublin", 'l'));
    println!("{}",liczba_wystapien2("ooooooooooo", 'o'));
    println!("{}",liczba_wystapien3("PODIWFIEUFBFBI", 'f'));
}
