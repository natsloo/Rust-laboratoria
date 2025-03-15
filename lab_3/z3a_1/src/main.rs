fn swap(a: &mut i32, b: &mut i32){
    let pom = *a;
    *a = *b;
    *b = pom;
}

fn main() {
    let mut a = 4;
    let mut b = 10;
    println!("Przed swapem a: {a}, b: {b}.");
    swap(&mut a, &mut b);
    println!("Po swapie a: {a}, b: {b}.");
}
