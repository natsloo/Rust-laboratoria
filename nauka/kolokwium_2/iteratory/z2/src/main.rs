use std::collections::HashMap;
fn unikalne(t: &[u32]) -> Vec<u32> {
    let mut u = vec![];
    let mut dodaj = true;
    for i in 0..t.len(){
        dodaj = true;
        for j in 0..t.len() {
            if t[i] == t[j] && i != j {
                dodaj = false;
            }
        }
        if dodaj {
            u.push(t[i]);
        }
    }
    u
}

fn f(t: &[u32]) {
    let a = t.into_iter().filter(|x| **x == 1).count();
    println!("{a}");
    let b: Vec<_> = t.into_iter().filter(|x| **x < 4).collect();
    println!("{:?}", b);
    let c: Vec<_> = t.into_iter().filter(|x| t.into_iter().filter(|y| **y == **x).count() < 2).collect();
    println!("{:?}", c);
}

fn main() {
    println!("{:?}", unikalne(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]));
    f(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]);
}
