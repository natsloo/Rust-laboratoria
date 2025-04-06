fn c_to_f(c: f64) -> f64 {
    32.0 + 9.0/5.0 * c
}

fn f_to_c(f: f64) -> f64 {
    5.0/9.0 * (f - 32.0)
}

fn main() {
    println!("{}", c_to_f(0.0));
    println!("{}", c_to_f(110.0));
    println!("{}", f_to_c(0.0));
    println!("{}", f_to_c(110.0));
}
