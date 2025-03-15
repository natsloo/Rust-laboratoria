fn f(x: f64) -> f64 {
    x * x - 4.0 * x - 5.0
}

fn fp(x: f64) -> f64 {
    2.0 * x - 4.0
}

fn met_newt(mut x0: f64, eps: f64, n:u128) -> f64 {
    for _ in 0..=n {
        let mut x_next = x0 - f(x0) / fp(x0);
        if (x_next - x0).abs() <= eps{
            return x_next;
        }
        x0 = x_next;
    }
    x0
}

fn main() {
    let x0 = 8.0;
    println!("{}",met_newt(x0, 0.01, 15));
}
