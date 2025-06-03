fn pow(t: &[u32]) -> Vec<u32> {
    let mut p = vec![];
    let mut poprzedni = t[0];
    let mut dodaj = true;
    for i in 1..t.len() {
        if t[i] == poprzedni {
            if dodaj {
                p.push(poprzedni);
                dodaj = false;
            }
            p.push(t[i])
        }
        else {
            poprzedni = t[i];
            dodaj = true;
        }
    }
    p
}

fn pow2(t: &[u32]) -> Vec<u32> {
    let mut p = vec![];
    if t.len() < 2 {
        return p;
    }
    if t[0] == t[1] {
        p.push(t[0]);
    }
    for i in 1..t.len() - 1 {
        if t[i] == t[i-1] || t[i] == t[i+1] {
            p.push(t[i]);
        }
    }
    if t[t.len()-2] == t[t.len()-1] {
        p.push(t[t.len()-1]);
    }
    p
}

fn pow3(t: &[u32]) {
    let a:Vec<_> = t.into_iter().zip(t.into_iter().skip(1)).filter(|(a, b)| **a == **b).map(|(a,b)| a).collect();
    println!("{:?}", a);
    let b:Vec<_> = t.into_iter().enumerate().skip(1).filter(|(i,v)| t[i-1] == **v).map(|(i,v)|v).collect();
    println!("{:?}", b);
    let c:Vec<_> = t.into_iter().enumerate().filter(|(i,v)| (*i > 0 as usize && t[i-1] == **v) || (*i < t.len()-1 && t[i+1] == **v)).map(|(i,v)|v).collect();
    println!("{:?}", c);


}

fn main() {
    println!("{:?}", pow3(&vec![1, 3, 4, 3, 3, 3, 3, 4, 1, 1, 6]));
    println!("{:?}", pow3(&vec![3, 3, 1, 1]));
    println!("{:?}", pow3(&vec![3]));
    println!("{:?}", pow3(&vec![]));
    //println!("{:?}", pow3(&vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]) == [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
}
