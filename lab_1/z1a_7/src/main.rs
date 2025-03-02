fn main() {
    let mut dana = 100;
    while dana > 0  {
        let ostatnia = dana % 10;
        print!("{ostatnia} ");
        dana /= 10;
    }
}
