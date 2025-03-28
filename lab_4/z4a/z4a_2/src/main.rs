fn rzymskie(napis: &str) -> i32 {
    let mut suma = 0;
    let mut poprzednia = 0;
    let n:Vec<char> = napis.chars().collect();

    for c in n {
        let mut obecna = 0;
        if c == 'I' {
            obecna = 1;
        }
        else if c == 'V' {
            obecna = 5;
        }
        else if c == 'X' {
            obecna = 10;
        }
        else if c == 'L' {
            obecna = 50;
        }
        else if c == 'C' {
            obecna = 100;
        }
        else if c == 'D' {
            obecna = 500;
        }
        else {
            obecna = 1000;
        }
        if obecna > poprzednia {
            suma += obecna - 2*poprzednia;
        }
        else {
            suma += obecna;
        }
        poprzednia = obecna;
    }
    suma
}

fn main() {
    let napis:&str = "MCMX";
    println!("{} = {}", napis, rzymskie(napis));
}

