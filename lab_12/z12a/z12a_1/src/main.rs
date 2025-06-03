
#[derive (Debug, Clone, PartialEq)]
struct RandGen {
    seed: i64
}
impl RandGen {
    fn new(seed: i64) -> Self {
        RandGen{
            seed
        }
    }
    fn change_seed(seed: &mut i64) {
        let a = 22695477;
        *seed = (a * *seed + 1) % 2_i64.pow(31);
    }

    fn gen_range(&mut self, min_rand: i64, max_rand: i64) -> i64 {
        RandGen::change_seed(&mut self.seed);
        self.seed % (max_rand - min_rand + 1) + min_rand
    }
}
#[derive (Debug, Clone, PartialEq)]
struct Urna<T> {
    v: Vec<T>,
    generator: RandGen
}
impl<T: Clone> Urna<T> {
    fn new(generator: RandGen) -> Self {
        Urna{
            v: Vec::new(),
            generator
        }
    }

    fn losuj_z_us(&mut self) -> Option<T> {
        if self.v.is_empty() {
            return None
        }
        let a = self.generator.gen_range(0,(self.v.len() - 1) as i64);
        let c = self.v.swap_remove(a as usize);
        Some(c)
    }
    fn losuj_bez_us(&mut self) -> Option<T>{
        if self.v.is_empty() {
            return None
        }
        let a = self.generator.gen_range(0,(self.v.len() - 1) as i64);
        Some(self.v[a as usize].clone())
    }
    fn doloz(&mut self, a: T){
        self.v.push(a);
    }
    fn rozmiar(&mut self) -> usize{
        self.v.len()
    }

}

fn main() {
    let mut urna = Urna::<char>::new(RandGen::new(123));

    let a: Option<char> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<char> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz('a');
    urna.doloz('b');
    urna.doloz('c');
    urna.doloz('d');

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<char> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<char> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<char> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}
