fn digit_sum(num: &mut i64) -> i64 {
    let mut result = 0;
    let len = num.to_string().len();
    let mut i = 0;
    while i < len {
        if result > 0 {
            print!(" + ");
        }
        let ostatnia = *num % 10;
        print!("{}", ostatnia);
        result += ostatnia;
        *num /= 10;
        i += 1;
    }
    print!(" = ");
    result
}


fn main() {
    println!("{}", digit_sum(&mut 123456));
}
