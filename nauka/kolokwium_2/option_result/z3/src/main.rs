fn divisors(number: Option<u32>) -> usize {
    if number.is_none() {
        panic!("number is none!");
    }

    let mut divisors:usize = 0;
    let n = number.unwrap();
    for i in 1..=n {
        if n % i == 0 {
           divisors += 1;
        }
    }
    divisors
}

fn main() {
    println!("{}", divisors(Some(932)));
    println!("{}", divisors(Some(86857)));
    println!("{}", divisors(None));
}
