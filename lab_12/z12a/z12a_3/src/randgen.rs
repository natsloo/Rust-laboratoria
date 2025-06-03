pub struct RandGen {
    seed: i64
}
impl RandGen {
    pub fn new(seed: i64) -> Self {
        RandGen{
            seed
        }
    }
    pub fn change_seed(seed: &mut i64) {
        let a = 22695477;
        *seed = (a * *seed + 1) % 2_i64.pow(31);
    }

    pub fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64 {
        RandGen::change_seed(&mut self.seed);
        self.seed % (max_rand - min_rand + 1) + min_rand
    }
}