use z13a_1::ulamek::Ulamek;

fn main() {
    let u1 = Ulamek::new(2,3);
    let u2 = Ulamek::new(1,5);
    println!("{}", u1 + u2);
    let u1 = Ulamek::new(4,8);
    let u2 = Ulamek::new(-4,8);
    println!("{}", u1 - u2);
    let u1 = Ulamek::new(10,-20);
    let u2 = Ulamek::new(-5,-15);
    println!("{}", u1 * u2);
    let u1 = Ulamek::new(1,3);
    let u2 = Ulamek::new(2,3);
    println!("{}", u1 / u2);
}