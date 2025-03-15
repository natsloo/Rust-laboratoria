fn change_seed(seed: &mut i128) {
    let a = 22695477;
    *seed = (a * *seed + 1) % 2_i128.pow(31);
}

fn rand(seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
    change_seed(seed);
    *seed % (max_rand - min_rand + 1) + min_rand
}

fn main() {
    let mut seed = 10;
    println!("{}",rand(&mut seed,0,10));
    println!("{}",rand(&mut seed,0,10));
    println!("{}",rand(&mut seed,0,10));
}
