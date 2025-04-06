fn main() {
    const N: usize = 50;
    let mut tablica= [0;N];
    for i in 0..N {
        tablica[i] = 100 % (i + 1);
    }
    println!("\n==================");
    for j in (0..tablica.len()).rev() {
        print!("{} ", tablica[j]);
    }
    println!("\n==================");
    let mut vector: Vec<_> = vec![];
    for k in 0..N {
        vector.push(100 % (k + 1));
    }
    for l in (0..vector.len()).rev(){
        print!("{} ", vector[l]);
    }
}
