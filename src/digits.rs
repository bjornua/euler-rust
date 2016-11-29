#[derive(Debug, PartialEq)]
pub struct Digits([u8]);
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub struct DigitsBuf(Vec<u8>);

use std::ops::{Add};

impl From<u64> for DigitsBuf {
    fn from(n: u64) -> Self {
        let mut n = n;
        let mut v: Vec<u8> = Vec::new();
        loop {
            v.push((n % 10) as u8);
            n = n / 10;
            if n == 0 {
                break;
            }
        }
        v.reverse();
        DigitsBuf(v)
    }
}
impl Digits {
    pub fn new<'a>(other: &[u8]) -> &'a Digits {
        unsafe { mem::transmute(other) }
    }
}

impl AsRef<Digits> for DigitsBuf {
    fn as_ref(&self) -> &Digits {
        Digits::new(self.0.as_ref())
    }
}

use std::ops::{Deref, Mul};
impl Deref for DigitsBuf {
    type Target = Digits;
    fn deref(&self) -> &Digits {
        self.as_ref()
    }
}

impl<'a> Add for &'a Digits {
    type Output = DigitsBuf;
    fn add(self, other: Self) -> Self::Output {
        let mut result = DigitsBuf(Vec::new());
        let mut left = self.into_iter().rev().cloned();
        let mut right = other.into_iter().rev().cloned();
        let mut carry = 0;
        loop {
            let sum = match (left.next(), right.next(), carry) {
                (None, None, 0) => break,
                (None, None, carry) => carry,
                (Some(a), Some(b), carry) => a + b + carry,
                (Some(a), None, carry) => a + carry,
                (None, Some(b), carry) => b + carry,
            };
            carry = sum / 10;
            result.0.push(sum % 10);
        }
        result.0.reverse();
        result
    }
}
use std::iter::repeat;
impl<'a> Mul for &'a Digits {
    type Output = DigitsBuf;
    fn mul(self, other: Self) -> Self::Output {
        let mut total = DigitsBuf(vec![0]);
        for (n, l) in self.into_iter().rev().cloned().enumerate() {
            let mut carry = 0;
            let mut mul_result = DigitsBuf(Vec::new());
            let mut right = other.into_iter().rev().cloned();
            mul_result.0.extend(repeat(0).take(n));
            loop {
                let sum = match (right.next(), carry) {
                    (None, 0) => break,
                    (None, carry) => carry,
                    (Some(a), carry) => a * l + carry,
                };
                carry = sum / 10;
                mul_result.0.push(sum % 10);
            }
            mul_result.0.reverse();
            total = total.add(&mul_result);
        }
        total
    }
}

impl<'a> IntoIterator for &'a Digits {
    type Item = &'a u8;
    // type IntoIter = ::std::iter::Rev<::std::slice::Iter<'a, u8>>;
    type IntoIter = ::std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

#[test]
fn test_add() {
    for a in 1..99 {
        for b in 1..99 {
            let c: DigitsBuf = (a + b).into();
            let a: DigitsBuf = a.into();
            let b: DigitsBuf = b.into();
            assert_eq!(a.add(&b), c);
        }
    }
}
#[test]
fn test_mul() {
    for a in 1..99 {
        for b in 1..99 {
            let c: DigitsBuf = (a * b).into();
            let a: DigitsBuf = a.into();
            let b: DigitsBuf = b.into();
            assert_eq!(a.mul(&b), c);
        }
    }
}
