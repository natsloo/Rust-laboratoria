struct Macierz {
    tab: Vec<Vec<f64>>,
    wys: usize,
    szer: usize,
}

impl Macierz {
    fn new(wys: usize, szer: usize, wyp: f64) -> Self {
        Macierz {
            tab: vec![vec![wyp; wys]; szer],
            wys,
            szer,
        }
    }

    fn zerowa(wys: usize, szer: usize) -> Self {
        Self::new(wys, szer, 0.0)
    }

    fn jednostkowa(wys: usize) -> Self {
        let mut tab = vec![vec![0.0; wys]; wys];
        for i in 0..wys {
            tab[i][i] = 1.0
        }
        Macierz {
            tab,
            wys,
            szer: wys
        }
    }

    fn element(&self, i_w: usize, i_k: usize) -> f64 {
        self.tab[i_w][i_k]
    }

    fn zmien_element(&mut self, i_w: usize, i_k: usize, w: f64) {
        self.tab[i_w][i_k] = w;
    }

    fn suma(m1: &Macierz, m2: &Macierz) -> Option<Macierz> {
        if m1.wys != m2.wys || m1.szer != m2.szer {
            return None
        }
        let mut tab = vec![vec![0.0; m1.wys]; m1.szer];
        for i in 0..m1.wys {
            for j in 0..m1.szer {
                tab[i][j] = m1.tab[i][j] + m2.tab[i][j];
            }
        }
        Some(Macierz {
            tab,
            wys: m1.wys,
            szer: m1.szer,
        })
    }

    fn wyswietl(&self) {
        if self.tab.is_empty() {
            return;
        }
        for i in 0..self.wys {
            for j in 0..self.szer{
                print!("{} ", self.tab[j][i]);
            }
            println!();
        }
    }
}

fn main() {
    let m1 = Macierz::new(2, 3, 1.5);
    println!("Macierz new(2, 3, 1.5):");
    m1.wyswietl();

    let m2 = Macierz::zerowa(3, 2);
    println!("Macierz zerowa(3, 2):");
    m2.wyswietl();

    let m3 = Macierz::jednostkowa(3);
    println!("Macierz jednostkowa(3):");
    m3.wyswietl();

    let val = m1.element(0, 1);
    println!("Element na pozycji (0, 1) w m1: {}", val);

    let mut m4 = Macierz::zerowa(2, 2);
    m4.zmien_element(1, 1, 9.0);
    println!("Po zmianie elementu (1, 1) na 9.0 w m4:");
    m4.wyswietl();

    let m5 = Macierz::new(2, 2, 2.0);
    let m6 = Macierz::new(2, 2, 3.0);
    let suma = Macierz::suma(&m5, &m6);
    match suma {
        Some(s) => {
            println!("Suma m5 i m6:");
            s.wyswietl();
        },
        None => println!("Nie można dodać macierzy m5 i m6."),
    }

    let m5 = Macierz::new(2, 2, 2.0);
    let m6 = Macierz::new(3, 2, 3.0);
    let suma = Macierz::suma(&m5, &m6);
    match suma {
        Some(s) => {
            println!("Suma m5 i m6:");
            s.wyswietl();
        },
        None => println!("Nie można dodać macierzy m5 i m6."),
    }
}