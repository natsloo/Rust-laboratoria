use crate::reka::Reka;
pub struct Gracz {
    reka: Reka
}

impl Gracz {
    pub fn new() -> Self {
        Gracz {
            reka: Reka::new()
        }
    }
    pub fn reka(&mut self) -> &mut Reka {
        &mut self.reka
    }
}