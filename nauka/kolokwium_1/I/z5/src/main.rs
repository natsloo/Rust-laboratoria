fn swap (a: &mut u32, b: &mut u32) {
    let pom;
    pom = *a;
    *a = *b;
    *b = pom;
}

fn sort(a: &mut u32, b: &mut u32, c: &mut u32) {
    if *a > *b {
        swap(a, b);
    }
    if *b > *c {
        swap(b, c);
    }
    if *a > *b {
        swap(a, b);
    }

}

fn main() {
    let mut a = 3;
    let mut b = 1;
    let mut c = 2;
    sort(&mut a,&mut b,&mut c);
    println!("{}, {}, {}", a, b, c)

}
