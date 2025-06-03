use crate::randgen::RandGen;

pub struct Urna<T> {
    v: Vec<T>,
    generator: RandGen
}
impl<T: Clone> Urna<T> {
    pub fn new(generator: RandGen) -> Self {
        Urna{
            v: Vec::new(),
            generator
        }
    }

    pub fn losuj_z_us(&mut self) -> Option<T> {
        if self.v.is_empty() {
            return None
        }
        let a = self.generator.gen_range(0,(self.v.len() - 1) as i64);
        let c = self.v.swap_remove(a as usize);
        Some(c)
    }
    pub fn losuj_bez_us(&mut self) -> Option<T>{
        if self.v.is_empty() {
            return None
        }
        let a = self.generator.gen_range(0,(self.v.len() - 1) as i64);
        Some(self.v[a as usize].clone())
    }
    pub fn doloz(&mut self, a: T){
        self.v.push(a);
    }
    pub fn rozmiar(&mut self) -> usize{
        self.v.len()
    }

}