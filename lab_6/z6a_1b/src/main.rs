fn main() {
    let mut v: Vec<_> = vec![];
    for c in 'a'..='z' {
        v.push(c);
    }
    println!("{:?}\n", v);

    let mut v: Vec<_> = vec![];
    for i in 1..=10 {
        v.push(i * i);
    }
    println!("{:?}\n", v);

    let mut v: Vec<_> = vec![];
    for i in 0..10 {
        v.push(2_i32.pow(i));
    }
    println!("{:?}\n", v);

    let mut v: Vec<_> = vec![];
    for i in 1..=20 {
        v.push(1.0/i as f64)
    }
    println!("{:?}\n", v);

    let mut v: Vec<_> = vec![];
    for i in 1..=100 {
        if i % 3 == 0 && i % 4 != 0 {
            v.push(i);
        }
    }
    println!("{:?}\n", v);

}
