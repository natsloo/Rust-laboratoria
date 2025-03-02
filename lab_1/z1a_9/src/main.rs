fn main() {
    let dana = 100;

    for a in 1..dana {
        for b in a + 1..dana {
            for c in b + 1..=dana {
                //println!("{a}, {b}, {c}")
                if a * a + b * b == c * c {
                    println!("({a}, {b}, {c})");
                }
            }
        }
    }
}
