fn co_drugi_znak(napis: &str) -> String {
    napis.chars().enumerate().filter(|(i,_)| i % 2 == 0).map(|(_, v)| v).collect()
}

fn main() {
    println!("{}", co_drugi_znak("Hello, world!"));
}
