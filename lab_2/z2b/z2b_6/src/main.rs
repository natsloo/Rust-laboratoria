// własność modulo dla iloczynu:
// (a * b) % p = [(a % p) * (b % p)] % p

// szybkie potęgowanie:
// x^n = x ∗ x^n − 1, jeśli n jest nieparzyste
// x^n = x^(n / 2) ∗ x^(n / 2), jeśli n jest parzyste

// x^n % p


fn pow_mod (x: u128, n: u128, p: u128) -> u128 {
    if n == 1 {
        return x%p
    }
    if n%2 == 0 {
        let t = pow_mod(x, n/2, p);
        t * t %p
    }
    else {
        x%p * pow_mod(x, n-1, p)
    }
}

fn main() {
    println!("{}",pow_mod(4564,544,54));
}
