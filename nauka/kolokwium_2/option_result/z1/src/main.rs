fn fraction(numerator: i32, denominator: i32) -> Option<f32> {
    if denominator == 0 {
        return None
    }
    Some(numerator as f32/denominator as f32)
}


fn main() {
    println!("{:?}", fraction(1,3));
    println!("{:?}", fraction(5,0));
    println!("{:?}", fraction(5,5));
    println!("{:?}", fraction(5,1));
    println!("{:?}", fraction(5,2));

}
