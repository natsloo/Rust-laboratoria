#[derive(PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    fn from_3u8(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            r,
            g,
            b,
        }
    }

    fn from_3percent(r: f64, g: f64, b: f64) -> Option<Rgb> {
        if r < 0.0 || r > 100.0 {
            return None
        }
        if g < 0.0 || g > 100.0 {
            return None
        }
        if b < 0.0 || b > 100.0 {
            return None
        }
        Some(Rgb {
            r: (255.0 * r/100.0) as u8,
            g: (255.0 * g/100.0) as u8,
            b: (255.0 * b/100.0) as u8,
        })
    }
    fn gray(percent: f64) -> Option<Rgb> {
        if percent < 0.0 || percent > 100.0 {
            return None
        }
        Some(Rgb {
            r: (255.0 * percent/100.0) as u8,
            g: (255.0 * percent/100.0) as u8,
            b: (255.0 * percent/100.0) as u8,
        })
    }
    fn white() -> Rgb {
        Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    fn black() -> Rgb {
        Rgb {
            r: 0,
            g: 0,
            b: 0,
        }
    }
    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }
    fn intensity(&self) -> f64 {
        ((self.r as f64) + (self.g as f64)  + (self.b as f64))/(255.0 * 3.0)
    }
    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    fn as_cmy_u8tuple(&self) -> (u8, u8, u8){
        (255 - self.r, 255 - self.g, 255 - self.b)
    }
}


fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0/3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));
}
