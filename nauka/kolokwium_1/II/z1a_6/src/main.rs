fn factorial(n: i64) -> i64 {
    let mut result = 1;
    let mut i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    result

}


fn main() {
    println!("{}", factorial(5));
    println!("{}", factorial(8));
    println!("{}", factorial(10));
}
