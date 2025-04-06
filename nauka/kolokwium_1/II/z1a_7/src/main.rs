fn reversed_digits(num: &mut i64) {
    let len = num.to_string().len();
    let mut i = 0;
    while i < len {
        let last = *num % 10;
        print!("{}", last);
        *num /= 10;
        i += 1;
    }
}


fn main() {
    reversed_digits(&mut 12839025412);
}
