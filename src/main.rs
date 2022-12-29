use std::ops::*;

const P: u64 = 7;

pub struct F(u64);

impl Add for F {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0 % P)
    }
}

impl Sub for F {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0 % P)
    }
}

impl Mul for F {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0 % P)
    }
}

impl Div for F {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if other.0 == 0 {
            panic!("Cannot divided by 0");
        }
        Self(self.0 * inv(other).0 % P)
    }
}

fn inv(x: F) -> F {
    let mut exp = P - 2;
    let mut tot = 1;
    let mut cur = x.0;
    while exp > 0 {
        let bit = exp & 1 == 1;
        tot = if bit {tot * cur % P} else {tot} ;
        cur = cur * cur % P;
        exp >>= 1;
    }
    F(tot)
}

#[test]
fn test_inv () {
    for i in 1..P-1 {
        assert_eq!((F(i)*inv(F(i))).0, 1);
    }
}

fn main() {
    println!("{}", (F(4)/ F(3)).0);
}
