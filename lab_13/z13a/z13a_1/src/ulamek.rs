use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,DivAssign,MulAssign};
use std::fmt;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub struct Ulamek {
    licznik: i64,
    mianownik: i64
}

impl Ulamek {
    fn gcd(a:i64, b:i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
    pub fn new(licznik: i64, mianownik: i64) -> Self {
        assert_ne!(mianownik, 0, "Mianownik nie moze byc zerem!");
        let gcd = Ulamek::gcd(licznik, mianownik);
        let (mut l, mut m) = (licznik/gcd, mianownik/gcd);
        if m < 0 {
            l = -l;
            m = -m;
        }
        Self {
            licznik: l,
            mianownik: m
        }
    }
    pub fn as_f64(&self) -> f64 {
        self.licznik as f64/self.mianownik as f64
    }
    pub fn licznik(&self) -> i64 {
        self.licznik
    }
    pub fn mianownik(&self) -> i64 {
        self.mianownik
    }
}

impl Add for Ulamek {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Ulamek::new(
            self.licznik * other.mianownik + other.licznik * self.mianownik,
            self.mianownik * other.mianownik
        )
    }
}

impl Sub for Ulamek {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Ulamek::new(
            self.licznik * other.mianownik - other.licznik * self.mianownik,
            self.mianownik * other.mianownik
        )
    }
}

impl Mul for Ulamek {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Ulamek::new(
            self.licznik * other.licznik,
            self.mianownik * other.mianownik
        )
    }
}

impl Div for Ulamek {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        assert_ne!(other.mianownik, 0, "Mianownik nie moze byc zerem!");
        Ulamek::new(
            self.licznik * other.mianownik,
            self.mianownik * other.licznik
        )
    }
}

impl AddAssign for Ulamek {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl SubAssign for Ulamek {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl MulAssign for Ulamek {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl DivAssign for Ulamek {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other
    }
}

impl PartialEq for Ulamek {
    fn eq(&self, other: &Self) -> bool {
        self.licznik == other.licznik && self.mianownik == other.mianownik
    }
}

impl Eq for Ulamek {}

impl fmt::Display for Ulamek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.licznik, self.mianownik)
    }
}

impl std::str::FromStr for Ulamek {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('/').collect();
        if parts.len() != 2 {
            return Err("Ulamek musi mieć format: licznik/mianownik.".to_string());
        }
        let licznik = parts[0].trim().parse::<i64>().map_err(|_| "Niepoprawny licznik!")?;
        let mianownik = parts[1].trim().parse::<i64>().map_err(|_| "Niepoprawny mianownik!")?;
        if mianownik == 0 {
            return Err(String::from("Mianownik nie moze byc zerem!"));
        }
        Ok(Ulamek::new(licznik, mianownik))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_f64() {
        let u1 = Ulamek::new(3, 4);
        assert_eq!(u1.as_f64(), 0.75);
    }

    #[test]
    fn test_add() {
        let u1 = Ulamek::new(1, 3);
        let u2 = Ulamek::new(1, 2);
        assert_eq!(u1 + u2, Ulamek::new(5, 6));
    }

    #[test]
    #[should_panic]
    fn test_zerowy_mianownik() {
        let _ = Ulamek::new(1, 0);
    }

    #[test]
    fn test_rozne_zapisy_tego_samego_ulamka() {
        assert_eq!(Ulamek::new(1, -3), Ulamek::new(-2, 6));
    }

    #[test]
    fn test_z_napisu_1() {
        let u1 = Ulamek::from_str("1/-3").unwrap();
        let u2 = Ulamek::from_str("-2/6").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(-1, 3));
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_1() {
        let _ = Ulamek::from_str("x/-3").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_2() {
        let _ = Ulamek::from_str("1/3/5").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_3() {
        let _ = Ulamek::from_str("/5").unwrap();
    }
}