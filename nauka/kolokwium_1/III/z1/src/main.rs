use std::collections::HashSet;                  // zostawione jako ciekawostka
fn powtorki_hash_set(t: &[i32]) -> Vec<i32> {   // ta funkcja robi wybiera z tablicy tylko wartości unikalne
    let mut set = HashSet::new();  // dodaje tylko raz liczby + w dowolnej kolejności, bo to HashSet
    for i in t {                          // nie o to chodzi w zadaniu, trochę przeciwieństwo polecenia
        set.insert(*i);
    }
    let result = set.into_iter().collect(); // HashSet -> Vec
    result
}

fn powtorki(t: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    let mut i = 1;
    while i < t.len() {
        if t[i - 1] == t[i] {
            result.push(t[i - 1]);
            while i < t.len() && t[i - 1] == t[i] {
                result.push(t[i]);
                i += 1;
            }
        }
        else {
            i += 1
        }
    }
    result
}

fn main() {
    let t = vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6, 5, 5, 9, 9, 4];
    println!("{:?}", powtorki(&t));
}
