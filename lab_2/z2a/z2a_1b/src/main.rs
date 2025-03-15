fn main() {
    let dana = 20;
    let mut a = 1;

    while a < dana {
        let mut b = a + 1;
        while b < dana {
            let mut c = b + 1;
            while c <= dana {
                if a * a + b * b == c * c {
                    println!("({a}, {b}, {c})");
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }

}
