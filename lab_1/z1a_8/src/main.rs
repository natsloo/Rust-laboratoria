fn main() {
    let mut dana = 789;
    let mut suma = 0;
    while dana > 0 {
        let ostatnia = dana % 10;
        suma += ostatnia;
        dana /= 10;
    }
    println!("suma = {suma}")
}
