fn powtorki(t: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    let mut i = 0;
    while i < t.len() {
        let mut count = 1;
        while i + count < t.len() && t[i] == t[i + count] {
            count += 1;
        }
        if count > 1 {
            result.extend(                  // extend() dodaje wszystkie elementy z podanego iteratora (lub kolekcji) do końca innej kolekcji, tu vectora
                std::iter::repeat(t[i]) // iterator repeat() powtarza jeden element bez końca
                    .take(count));          // z tego powodu dodajemy take(), i podajemy mu wyliczoną liczbę wystąpień
        }
        i += count;
    }
    result
}

fn main() {
    let t = vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6, 5, 5, 9, 9, 4];
    println!("{:?}", powtorki(&t));
}
