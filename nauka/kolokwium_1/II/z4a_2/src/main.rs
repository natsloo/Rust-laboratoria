fn rzymskie(napis: &str) -> i32 {
    let mut suma = 0;
    let chars = napis.chars();
    let mut poprzednia = 0;
    for c in chars {
        let mut obecna = 0;
        match c {
            'I' => obecna = 1,
            'V' => obecna = 5,
            'X' => obecna = 10,
            'L' => obecna = 50,
            'C' => obecna = 100,
            'D' => obecna = 500,
            'M' => obecna = 1000,
            _ => return -1
        }
        if obecna > poprzednia {
            suma += obecna - 2 * poprzednia;
        }
        else {
            suma += obecna;
        }
        poprzednia = obecna;
    }
    suma
}

fn main() {
    println!("{}", rzymskie("XLIX"));
}
