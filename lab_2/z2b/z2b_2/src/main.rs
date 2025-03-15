fn collatz_iterations(mut c0: u128  ) -> i128 {
    let mut iter:i128 = 0;
    while c0 != 1 {
        if c0 % 2 == 0 {
            //c0 = 1/2 * c0; - 1/2 w Rust (dla liczb całkowitych) to dzielenie całkowite, więc wynik to 0.
            c0 /= 2;
        }
        else {
            c0 = 3 * c0 + 1;
        }
        iter += 1;
    }
    iter
}

fn main() {
    let c0 = 27;
    println!("{}", collatz_iterations(c0));
}
