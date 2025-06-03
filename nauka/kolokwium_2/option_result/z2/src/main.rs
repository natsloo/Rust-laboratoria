fn position(element: i32, array: &[i32]) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == element {
            return Some(i)
        }
    }
    None
}

fn main() {
    let arr = [1,2,3,4,5,6,7,8,9];
    println!("{:?}", position(5, &arr));
    println!("{:?}", position(0, &arr));
    println!("{:?}", position(1, &arr));
    println!("{:?}", position(9, &arr));
}
