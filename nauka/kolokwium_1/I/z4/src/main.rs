fn avg(tab: &[u32]) -> f32 {
    let mut suma:u32 = 0;
    for i in 0..tab.len(){
        suma += tab[i];
    }
    suma as f32/tab.len() as f32
}

fn avg2(tab: &[u32]) -> f32 {
    tab.iter().sum::<u32>() as f32/tab.len() as f32
}

fn main() {
    let mut tab: [u32;8] = [0;8];
    for i in 0..8 {
        tab[i] = (i + i * 3) as u32;
    }
    println!("avg: {}", avg(&tab));
    println!("avg2: {}", avg2(&tab));
    let tab: [u32; 3] = [1, 2, 4];
    println!("avg: {}", avg(&tab));
    println!("avg2: {}", avg2(&tab));

}
