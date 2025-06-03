fn miejsce_zerowe(a: f32, b: f32, c: f32) -> (Option<f32>,Option<f32>) {
    let delta = b * b - 4.0 * a * c;
    if delta >= 0.0 {
        if delta == 0.0 {
            return (Some(-b/(2.0 * a)), None)
        }
        else {
            return (Some((-b - delta.sqrt())/(2.0 * a)), Some((-b + delta.sqrt())/(2.0 * a)))
        }
    }
    else {
        return (None, None)
    }
}

fn main() {
     println!("{:?}", miejsce_zerowe(3.0,6.0,10.0));
     println!("{:?}", miejsce_zerowe(1.0,-4.0,-5.0));
     println!("{:?}", miejsce_zerowe(-5.0,-4.0,1.0));
     println!("{:?}", miejsce_zerowe(1.0,-1.0,1.0));
     println!("{:?}", miejsce_zerowe(1.0,2.0,1.0));
}
