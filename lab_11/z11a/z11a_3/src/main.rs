use std::io;
fn wakacyjne_slowa(word: String) -> usize {
    let wakacje = ['w','a','k','a','c','j','e'];
    let mut indeks = 0;
    let mut znaleziono = 0;
    for ch in word.chars(){
        if ch == wakacje[indeks] {
            indeks += 1;
            if indeks == wakacje.len() {
                znaleziono += 1;
                indeks = 0;
            }
        }
    }
    let wykorzystane_literki = znaleziono * wakacje.len();
    word.len() - wykorzystane_literki
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        println!("{}",wakacyjne_slowa(input))
    }
}
