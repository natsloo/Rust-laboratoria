fn main() {
    let a: Vec<_> = ('a'..='z').collect();
    println!("{:?}\n", a);

    let b: Vec<_> = (1..=10).map(|x| {x * x}).collect();
    println!("{:?}\n", b);

    let c: Vec<_> = (0..10).map(|x| {2_i32.pow(x)}).collect();
    println!("{:?}\n", c);

    let d: Vec<_> = (1..=20).map(|x| {1.0/x as f64}).collect();
    println!("{:?}\n", d);

    let e: Vec<_> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("{:?}\n", e);

}
