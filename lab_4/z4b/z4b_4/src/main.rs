fn na_rzymskie(mut liczba: i32) -> String {
    let mut result = String::new();
    let symbols = [(1000,"M"), (900, "CM"), (500, "D"), (400,"CD"),
        (100,"C"), (90,"XC"), (50,"L"), (40,"XL"), (10, "X"), (9, "IX"),
        (5, "V"), (4, "IV"), (1, "I")];

    for (value,symbol) in symbols {
        while liczba >= value {
            result.push_str(symbol);
            liczba -= value;
        }
    }

    result
}

fn main() {
    let liczba = 1910;
    println!("{} -> {}", liczba, na_rzymskie(liczba));
}
