fn main() {
    let dana = 6;
    let mut i = 1;
    let mut silnia = 1;
    while i <= dana {
        silnia *= i;
        i+=1;
    }
    println!("{dana}! = {silnia}.");
}
