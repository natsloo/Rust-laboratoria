fn liczba_wystapien(napis: &str, znak: char) -> i32 {
    let mut counter: i32 = 0;
    let n: Vec<char> = napis.chars().collect();
    for c in n {
        if c == znak{
            counter+=1
        }
    }
    counter
}


fn main() {
    let napis:&str = "Ala ma kota.";
    let znak: char = 't';
    println!("znak '{}' x{} w '{}'", znak, liczba_wystapien(napis,znak), napis);
}
