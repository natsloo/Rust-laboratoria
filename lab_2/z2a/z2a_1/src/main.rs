fn main() {
    let dana = 10;
    let mut i = 1;
    let mut silnia = 1;
    loop {
        silnia *= i;
        i+=1;
        if i == dana + 1 {
            break;
        }
    }
    println!("{}", silnia)
}
