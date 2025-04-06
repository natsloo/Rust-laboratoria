fn main() {
    let hh1:i32 = 18;
    let mm1 = 24;
    let ss1 = 15;

    let hh2 = 21;
    let mm2 = 37;
    let ss2 = 42;

    let total_seconds1 = ss1 + mm1 * 60 + hh1 * 3600;
    let total_seconds2 = ss2 + mm2 * 60 + hh2 * 3600;
    let diff = (total_seconds1 - total_seconds2).abs();

    let rh = diff / 3600;
    let pozostale_sekundy = diff % 3600;
    let rm = pozostale_sekundy / 60 ;
    let rs = pozostale_sekundy % 60;

    println!("{}:{}:{}",rh, rm, rs);
}
