
fn d3(p1: (f32,f32,f32), p2: (f32,f32,f32)) -> f32{
    let a = (p1.0 - p2.0).powf(2.0);
    let b = (p1.1 - p2.1).powf(2.0);
    let c = (p1.2 - p2.2).powf(2.0);
    (a+b+c).sqrt()
}

fn main() {
    println!("{}", d3((1.0,3.0,5.0), (2.0,4.0,6.0)));
}
