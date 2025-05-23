use std::io;

fn wk_word(word: String) -> bool {
    let mut correct= false;
    let w_sum = word.chars().filter(|x| *x == 'w').count();
    let k_sum = word.chars().filter(|x| *x == 'k').count();;
    if w_sum == k_sum{
        correct = true;
    }
    correct
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        println!("{}", wk_word(input));
    }
}
