
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
#[derive (PartialEq, Clone)]
enum Moneta {
    Orzel,
    Reszka
}

fn testy_i32(){
    let mut urna = Urna::<i32>::new(RandGen::new(123));

    let a: Option<i32> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(1);
    urna.doloz(2);
    urna.doloz(3);
    urna.doloz(4);

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<i32> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<i32> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<i32> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}

fn testy_bool(){
    let mut urna = Urna::<bool>::new(RandGen::new(123));

    let a: Option<bool> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());

    urna.doloz(true);
    urna.doloz(false);

    println!("{:?}", urna.rozmiar() == 2);
    let y: Option<bool> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let x: Option<bool> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    println!("{:?}", x != y);
    let z: Option<bool> = urna.losuj_z_us();
    println!("{:?}", z.is_some());
    println!("{:?}", urna.rozmiar() == 0);
}

fn testy_string(){
    let mut urna = Urna::<String>::new(RandGen::new(123));

    let a: Option<String> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<String> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(String::from("a"));
    urna.doloz(String::from("b"));
    urna.doloz(String::from("c"));
    urna.doloz(String::from("d"));

    println!("{:?}", urna.rozmiar() == 4);
    let y: Option<String> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    let x: Option<String> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 3);
    println!("{:?}", x != y);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 2);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 1);
    urna.losuj_z_us();
    println!("{:?}", urna.rozmiar() == 0);
    let z: Option<String> = urna.losuj_z_us();
    println!("{:?}", z.is_none());
    println!("{:?}", urna.rozmiar() == 0);
}

fn testy_moneta(){
    let mut urna = Urna::<Moneta>::new(RandGen::new(123));

    let a: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", a.is_none());
    let a: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", a.is_none());


    urna.doloz(Moneta::Orzel);
    urna.doloz(Moneta::Reszka);

    println!("{:?}", urna.rozmiar() == 2);
    let y: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", y.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let x: Option<Moneta> = urna.losuj_bez_us();
    println!("{:?}", x.is_some());
    println!("{:?}", urna.rozmiar() == 1);
    let z: Option<Moneta> = urna.losuj_z_us();
    println!("{:?}", z.is_some());
    println!("{:?}", urna.rozmiar() == 0);
}

fn main() {
    println!("===Testy: i32===\n");
    testy_i32();
    println!("\n===Testy: bool===\n");
    testy_bool();
    println!("\n===Testy: string===\n");
    testy_string();
    println!("\n===Testy: moneta===\n");
    testy_moneta();
}
