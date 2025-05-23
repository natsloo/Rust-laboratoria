pub struct RandGen{}

impl RandGen {
    pub fn new() -> Self {
        RandGen{}
    }
    pub fn change_seed(&self, seed: &mut i128) {
        let a = 22695477;
        *seed = (a * *seed + 1) % 2_i128.pow(31);
    }

    pub fn rand(&self, seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
        self.change_seed(seed);
        *seed % (max_rand - min_rand + 1) + min_rand
    }
}