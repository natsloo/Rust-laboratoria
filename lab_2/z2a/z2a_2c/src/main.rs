fn f(x: f64) -> f64 {
    x * x - 4.0 * x - 5.0
}

fn fp(x: f64) -> f64 {
    2.0 * x - 4.0
}

fn met_newt(x0: f64, eps: f64, n: u128, mut iter: u128) -> f64 {
    let x_next = x0 - f(x0) / fp(x0);
    if (x_next - x0).abs() < eps || iter >= n {
        x_next //return x_next;
    }
    else {
        iter += 1;
        met_newt(x_next, eps, n, iter) //return met_newt(x_next, eps, n, iter);
    }
}

fn main() {
    let x = 8.0;
    println!("{}", met_newt(x, 0.05, 20, 0));
}
