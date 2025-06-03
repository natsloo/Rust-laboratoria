use std::collections::HashMap;
#[derive(PartialEq,Eq,Hash,Debug)]
enum Ocena {
    A,
    B,
    C,
    D,
    E,
    F
}
impl Ocena {
    fn to_f32(&self) -> f32{
        match self{
            Ocena::A => 5.0,
            Ocena::B => 4.0,
            Ocena::C => 3.0,
            Ocena::D => 2.0,
            Ocena::E => 1.0,
            Ocena::F => 0.0
        }
    }
}
struct Student {
    imie: String,
    nazwisko: String,
    oceny: Vec<Ocena>
}
impl Student {
    fn new(imie: String, nazwisko: String, oceny: Vec<Ocena>) -> Self {
        Self {
            imie,
            nazwisko,
            oceny
        }
    }
}

struct Dziennik {
    studenci: Vec<Student>
}
impl Dziennik {
    fn new() -> Self {
        Self {
            studenci: vec![]
        }
    }
    fn dodaj_studenta(&mut self, imie: String, nazwisko: String, oceny: Vec<Ocena>) {
        let s = Student::new(imie, nazwisko, oceny);
        self.studenci.push(s)
    }
    fn dodaj_ocene(&mut self, nazwisko: &str, ocena: Ocena) {
          for s in &mut self.studenci {
              if s.nazwisko == nazwisko {
                  s.oceny.push(ocena);
                  break;
              }
          }
    }
    fn srednia_ocen(&self, nazwisko: &str) -> Option<f32> {
        let mut suma = 0.0;
        let mut dlugosc = 0;
        for s in &self.studenci {
            if s.nazwisko == nazwisko {
                if s.oceny.is_empty(){
                    return None
                }
                else {
                    dlugosc = s.oceny.len();
                    for o in &s.oceny {
                        suma += o.to_f32();
                    }
                }
            }
        }
        Some(suma/dlugosc as f32)
    }
    fn najlepsi_studenci(&self, min_avg: f32) -> Vec<String> {
        let mut v = vec![];
        for s in &self.studenci {
            if self.srednia_ocen(&s.nazwisko) >= Some(min_avg) {
                v.push(s.nazwisko.clone() + &" ".to_string() + &s.imie);
            }
        }
        v
    }
    fn statystyka_ogolna_ocen(&self) -> HashMap<Ocena, usize> {
        let hm: HashMap<Ocena, usize>;
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        let mut e = 0;
        let mut f = 0;
        for s in &self.studenci {
            for o in &s.oceny {
                match o {
                    Ocena::A => a += 1,
                    Ocena::B => b += 1,
                    Ocena::C => c += 1,
                    Ocena::D => d += 1,
                    Ocena::E => e += 1,
                    Ocena::F => f += 1
                }
            }
        }
        hm = HashMap::from(
            [(Ocena::A, a),
                (Ocena::B, b),
                (Ocena::C, c),
                (Ocena::D, d),
                (Ocena::E, e),
                (Ocena::F, f)]
        );
        hm
    }
}


fn main() {
    let mut d = Dziennik::new();
    let oceny = vec![Ocena::A, Ocena::A, Ocena::B, Ocena::C];
    let oceny2 = vec![Ocena::C, Ocena::B, Ocena::B, Ocena::A];
    let oceny3 = vec![Ocena::F, Ocena::E, Ocena::E, Ocena::D];
    d.dodaj_studenta("Marcin".to_string(), "Kowalski".to_string(), oceny);
    d.dodaj_studenta("Anna".to_string(), "Nowak".to_string(), oceny2);
    d.dodaj_studenta("Tomasz".to_string(), "Zieliński".to_string(), oceny3);
    println!("{:?}", d.srednia_ocen("Zieliński"));
    d.dodaj_ocene("Zieliński",Ocena::A);
    println!("{:?}", d.srednia_ocen("Zieliński"));
    println!("{:?}", d.najlepsi_studenci(4.25));
    println!("{:?}", d.statystyka_ogolna_ocen());
}
