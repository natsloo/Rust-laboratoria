fn letters() -> Vec<char> {
    ('a'..='z').collect()
}

fn squares() -> Vec<u8> {
    //(1..=10).zip(1..=10).map(|(a,b)| a*b).collect();
    (1..=10).map(|a| a*a).collect()
}

fn powers() -> Vec<i32> {
    (1..=10).map(|x| 2_i32.pow(x)).collect()
}

fn invert() -> Vec<f32> {
    (1..=20).map(|x| 1.0/x as f32).collect()
}

fn three_not_4() -> Vec<i32> {
    (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect()
}

fn main() {
    println!("{:?}", letters());
    println!("{:?}", squares());
    println!("{:?}", powers());
    println!("{:?}", invert());
    println!("{:?}", three_not_4());
}
