fn swap(a: &mut i32, b: &mut i32){
    let pom = *a;
    *a = *b;
    *b = pom;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32){
    if *a > *b {
        swap(a,b)
    }
    if *b > *c {
        swap(b,c)
        }
    if *a > *b {
        swap(b,a)
        }
}

fn main() {
    let mut a = 3;
    let mut b = 2;
    let mut c = 1;
    sort(&mut a,&mut b,&mut c);
    println!("a: {a}, b: {b}, c: {c}");
}
