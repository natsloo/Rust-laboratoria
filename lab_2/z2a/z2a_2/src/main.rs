fn f(x: f64) -> f64 {
    x * x - 3.0 * x + 2.0
}

fn fp(x: f64) -> f64 {
    2.0 * x - 3.0
}

fn met_newt(x0: f64, eps: f64, n: u128) -> f64 {
    let mut x = x0;
    let mut iter = 0;

    loop {
        let x_next = x - f(x) / fp(x);
        if (x_next - x).abs() < eps || iter >= n {
            return x_next;
        }
        x = x_next;
        iter+=1;
    }
}

fn main() {
    let x0 = 0.5;
    println!("{}",met_newt(x0, 0.01, 15));
}
