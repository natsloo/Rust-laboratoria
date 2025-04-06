fn every_second_letter(s: &str) -> String {
    s.chars().enumerate().filter(|(i,_)| i % 2 == 0).map(|(_,v)| v).collect()
}

fn every_second_letter2(s: &str) -> String {
    s.chars().step_by(2).collect()
}

fn main() {
    println!("{}", every_second_letter("Konstantynopolitańczykowianeczka"));
    println!("{}", every_second_letter2("dziewięćsetdziewięćdziesięciodziewięciotysięcznik"))
}
