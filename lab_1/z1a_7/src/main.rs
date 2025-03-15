fn main() {
    let mut dana = 1234567;
    while dana > 0  {
        let ostatnia = dana % 10;
        print!("{ostatnia} ");
        dana /= 10;
    }
}
