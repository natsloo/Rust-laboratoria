fn f(x: f64) -> f64 {
    x * x - 3.0 * x + 2.0
}

fn fp(x: f64) -> f64 {
    2.0 * x - 3.0
}

fn met_newt(mut x0: f64, eps: f64, n: u128) -> f64 {
    let mut iter = 0;
    let mut x_next = x0 - f(x0) / fp(x0);
    while (x_next - x0).abs() >= eps && iter < n {
        x0 = x_next;
        x_next = x0 - f(x0) / fp(x0);
        iter += 1;
    }
    x0 //return x0;
}

fn main() {
    let x0 = 8.0;
    println!("{}",met_newt(x0, 0.01, 15));
}
