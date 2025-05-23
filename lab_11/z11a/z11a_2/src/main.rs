use std::io;

fn wakacje(word: String) -> i32 {
    let mut w = 0;
    let mut a = 0;
    let mut k = 0;
    let mut c = 0;
    let mut j = 0;
    let mut e = 0;

    for ch in word.chars() {
       match ch {
           'w' => w += 1,
           'a' => a += 1,
           'k' => k += 1,
           'c' => c += 1,
           'j' => j += 1,
           'e' => e += 1,
           _ => {}
       }
    }

    let count = [w, a/2, k, c, j, e];
    count.into_iter().min().unwrap()
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        println!("{}", wakacje(input));
    }
}
