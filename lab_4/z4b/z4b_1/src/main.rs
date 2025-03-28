fn co_drugi_znak(napis: &str) -> String {
    let nowy= napis.chars().step_by(2).collect();
    nowy
}

fn main() {
    let napis:&str = "Hello, world!";
    let napis2:&str = "abcdefghijk";
    println!("{} -> {}", napis, co_drugi_znak(napis));
    println!("{} -> {}", napis2, co_drugi_znak(napis2));
}
