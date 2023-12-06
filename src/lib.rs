use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
struct Mytime {
    hour: usize,
    minute: usize,
    second: usize,
}

impl Mytime {
    fn total_seconds(&self) -> usize {
        self.hour * 3600 + self.minute * 60 + self.second
    }
    fn normalize(&mut self) {
        self.minute += self.second / 60;
        self.second %= 60;
        self.hour += self.minute / 60;
        self.minute %= 60;
    }
}

impl PartialEq for Mytime {
    fn eq(&self, other: &Self) -> bool {
        self.total_seconds() == other.total_seconds()
    }
}

impl Display for Mytime {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl Add for Mytime {
    type Output = Self;
    fn add(self, other: Self) -> Mytime {
        let mut output: Mytime = Mytime {
            hour: 0,
            minute: 0,
            second: self.total_seconds() + other.total_seconds(),
        };
        output.normalize();
        output
    }
}

impl AddAssign for Mytime {
    fn add_assign(&mut self, other: Self) {
        self.second += other.total_seconds();
        self.normalize();
    }
}

impl Sub for Mytime {
    type Output = Self;
    fn sub(self, other: Self) -> Mytime {
        let s = self.total_seconds();
        let o = other.total_seconds();
        let mut output: Mytime;
        if s >= o {
            output = Mytime {
                hour: 0,
                minute: 0,
                second: s - o,
            };
        } else {
            output = Mytime {
                hour: 0,
                minute: 0,
                second: o - s,
            };
        }
        output.normalize();
        output
    }
}

impl SubAssign for Mytime {
    fn sub_assign(&mut self, other: Self) {
        let s = self.total_seconds();
        let o = other.total_seconds();
        if s >= o {
            self.hour = 0;
            self.minute = 0;
            self.second = s - o;
        } else {
            self.hour = 0;
            self.minute = 0;
            self.second = o - s;
        }
        self.normalize();
    }
}

impl Mul<usize> for Mytime {
    type Output = Self;
    fn mul(self, int: usize) -> Self {
        let mut output: Mytime = Mytime {
            hour: 0,
            minute: 0,
            second: self.total_seconds() * int,
        };
        output.normalize();
        output
    }
}

impl MulAssign<usize> for Mytime {
    fn mul_assign(&mut self, other: usize) {
        self.hour *= other;
        self.minute *= other;
        self.second *= other;
        self.normalize();
    }
}

impl Div<usize> for Mytime {
    type Output = Self;
    fn div(self, other: usize) -> Self {
        if other == 0 {
            panic!("Cannot divide by zero-valued `Mytime`!");
        }
        let mut output: Mytime = Mytime {
            hour: 0,
            minute: 0,
            second: self.total_seconds() / other,
        };
        output.normalize();
        output
    }
}

impl Div for Mytime {
    type Output = f32;
    fn div(self, other: Self) -> Self::Output {
        self.total_seconds() as f32 / other.total_seconds() as f32
    }
}

impl DivAssign<usize> for Mytime {
    fn div_assign(&mut self, other: usize) {
        if other == 0 {
            panic!("Cannot divide by zero-valued `Mytime`!");
        }
        let s = self.total_seconds();
        self.hour = 0;
        self.minute = 0;
        self.second = s / other;
        self.normalize();
    }
}

#[cfg(test)]
mod tests {
    use crate::Mytime;
    use rstest::*;
    #[fixture]
    pub fn fixture() -> [Mytime; 2] {
        [
            Mytime {
                hour: 1,
                minute: 23,
                second: 45,
            },
            Mytime {
                hour: 0,
                minute: 0,
                second: 0,
            },
        ]
    }
    #[rstest]
    fn test_init(fixture: [Mytime; 2]) {
        let a = &fixture[0];
        let b = &fixture[1];
        assert_eq!(a.hour, 1);
        assert_eq!(a.minute, 23);
        assert_eq!(a.second, 45);
        assert_eq!(b.hour, 0);
        assert_eq!(b.minute, 0);
        assert_eq!(b.second, 0);
    }
    #[rstest]
    fn test_total_seconds(fixture: [Mytime; 2]) {
        let a = &fixture[0];
        let b = &fixture[1];
        assert_eq!(a.total_seconds(), 3600 + 23 * 60 + 45);
        assert_eq!(b.total_seconds(), 0);
    }
    #[rstest]
    fn test_eq(fixture: [Mytime; 2]) {
        let a = fixture[0].clone();
        let mut b = fixture[1].clone();
        assert_ne!(a, b);
        b.hour = 1;
        assert_ne!(a, b);
        b.minute = 23;
        assert_ne!(a, b);
        b.second = 45;
        assert_eq!(a, b);
    }
    #[rstest]
    fn test_str(fixture: [Mytime; 2]) {
        let a = &fixture[0];
        let b = &fixture[1];
        assert_eq!(a.to_string(), "01:23:45".to_string());
        assert_eq!(b.to_string(), "00:00:00".to_string());
    }
    #[rstest]
    fn test_normalize(fixture: [Mytime; 2]) {
        let a = fixture[0].clone();
        let mut b = fixture[1].clone();
        b.second = a.total_seconds();
        assert_eq!(b.to_string(), "00:00:5025".to_string());
        assert_ne!(b.to_string(), a.to_string());
        assert_eq!(b, a);
        b.normalize();
        assert_eq!(b.to_string(), a.to_string());
    }
    #[rstest]
    fn test_add(fixture: [Mytime; 2]) {
        let mut a = fixture[0].clone();
        let b = fixture[1].clone();
        let c = a.clone() + b.clone();
        assert_eq!(c, a);
        let d = c.clone() + a.clone();
        assert_eq!(d.to_string(), "02:47:30");
        a += c;
        assert_eq!(a, d);
    }
    #[rstest]
    fn test_mul(fixture: [Mytime; 2]) {
        let mut a = fixture[0].clone();
        let b = a.clone() * 2;
        assert_eq!(b.to_string(), "02:47:30");
        let c = b * 3;
        assert_eq!(c.to_string(), "08:22:30");
        a *= 2;
        assert_eq!(a.to_string(), "02:47:30");
    }
    #[rstest]
    fn test_sub(fixture: [Mytime; 2]) {
        let mut a = fixture[0].clone();
        let mut b = fixture[1].clone();
        b.hour = 4;
        let c = b.clone() - a.clone();
        assert_eq!(c.to_string(), "02:36:15".to_string());
        let d = c.clone() - a.clone();
        assert_eq!(d.to_string(), "01:12:30".to_string());
        let e = a.clone() - c.clone();
        assert_eq!(d, e);
        b -= a.clone();
        assert_eq!(b.to_string(), "02:36:15".to_string());
        a -= b.clone();
        assert_eq!(a.to_string(), "01:12:30".to_string());
    }
    #[rstest]
    fn test_div(fixture: [Mytime; 2]) {
        let mut a = fixture[0].clone();
        let mut b = fixture[1].clone();
        b.minute = 20;
        let c = a.clone() / b;
        assert_eq!(c, 4.1875);
        let d = a.clone() / 3;
        assert_eq!(d.to_string(), "00:27:55".to_string());
        a /= 4;
        assert_eq!(a.to_string(), "00:20:56".to_string());
        a /= 2;
        assert_eq!(a.to_string(), "00:10:28".to_string());
    }
    #[rstest]
    #[should_panic]
    fn test_panic(fixture: [Mytime; 2]) {
        let _ = fixture[0].clone() / 0;
    }
}
