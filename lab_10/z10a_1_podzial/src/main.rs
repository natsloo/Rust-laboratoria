use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use z10a_1_podzial::gra::Gra;
fn startowy_seed() -> i128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("blad generacji seedu startowego!").as_millis() as i128
}

fn main() {
    let mut g = Gra::new();
    g.graj(&mut startowy_seed())
}