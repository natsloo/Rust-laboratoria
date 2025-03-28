fn szyfruj2(napis: String, klucz: i32) -> String {
    let n:Vec<char> = napis.chars().collect();
    let dlugosc_napisu = n.len();
    let mut napis2 = String::from("");
    let mut napis3 = String::from("");
    let mut napis4 = String::from("");
    if (dlugosc_napisu as i32) > klucz {
        let mut counter = 0;
        for c in n {
            if counter < klucz {
                napis2.push(c);
                counter+=1;
            }
            else {
                napis3.push(c);
            }

        }
        napis4 = napis2.chars().rev().collect();
        napis4 += &napis3;

    }
    else {
        napis2 = napis;
        napis4 = napis2.chars().rev().collect();
    }
    napis4
}

fn szyfruj(napis: String, klucz: usize) -> String {
    let mut n:Vec<char> = napis.chars().collect();
    let dlugosc_napisu = n.len();
    let mut i:usize = 0;
    let wynik:String;
    while i + klucz <= dlugosc_napisu {
        n[i..i + (klucz as usize)].reverse();
        i+=klucz;
    }
    wynik = n.iter().collect();
    wynik
}

fn main() {
    let napis = String::from("kot Mruczek");
    let klucz = 2;
    println!("{} -> {}",napis.clone(), szyfruj(napis,klucz));
}
