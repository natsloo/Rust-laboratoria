fn ascii_chart() {
    println!("{:^5}{}", "Kod", "Znak");
    println!("------------");
    for i in 33..=126 {
        println!("{:^5} {}", i, i as u8 as char);
        //println!("{}: {}", i, i as u8 as char);
    }
}

fn main() {
    ascii_chart();
}

// i ma w pętli domyślny typ i32
// char w Ruście to u32
// ASCII to wartości u8

// Rust nie pozwala na bezpośrednie rzutowanie u32 -> char,
// ponieważ nie wszystkie wartości u32 reprezentują poprawne znaki Unicode.
// char w Rust przechowuje wyłącznie poprawne wartości Unicode
// (czyli od 0x0000 do 0x10FFFF bez zakresu 0xD800–0xDFFF, który jest zarezerwowany dla par zastępczych w UTF-16).