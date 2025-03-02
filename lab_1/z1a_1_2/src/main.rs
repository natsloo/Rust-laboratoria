fn main() {
    let rok = 2024;
    let miesiac = 11;
    let mut przestepny = false;

    if rok % 4 == 0 {
        if rok % 100 == 0 {
            if rok %400 == 0 {
                println!("Rok {rok} jest przestępny.");
                przestepny = true;
            }
            else {
                println!("Rok {rok} nie jest przestępny.");
            }
        }
        else {
            println!("Rok {rok} jest przestępny.");
            przestepny = true;
        }
    }
    else {
        println!("Rok {rok} nie jest przestępny.");
    }

    if miesiac % 2 != 0 {
        if miesiac > 8 {
            println!("Miesiac {miesiac} ma 30 dni.");
        }
        else {
            println!("Miesiac {miesiac} ma 31 dni.");
        }
    }
    else if miesiac == 8 {
        println!("Miesiac {miesiac} ma 31 dni.");
    }
    else if miesiac == 2 && przestepny {
        println!("Miesiac {miesiac} ma 29 dni.");
    }
    else if miesiac == 2 && !przestepny {
        println!("Miesiac {miesiac} ma 28 dni.");
    }
    else {
        if miesiac > 8 {
            println!("Miesiac {miesiac} ma 31 dni.");
        }
        else {
            println!("Miesiac {miesiac} ma 30 dni.");
        }
    }
}
