fn d2(p1: (f32,f32), p2: (f32,f32)) -> f32 {
    let a = (p2.0 - p1.0).powf(2.0);
    let b =  (p2.1 - p1.1).powf(2.0);
    (a+b).sqrt()
}
fn main() {
    let p1 = (2.0,5.0);
    let p2 = (3.0,6.0);
    println!("{}", d2(p1,p2));
    println!("{}", d2((6.0,3.0), (8.0,4.0)))
}
