use digits::{DigitsBuf, self};
use std::ops::Mul;


pub fn factorial(n: u64) -> DigitsBuf {
    let mut digits = digits::DigitsBuf::from(1);
    for m in 2..(n+1) {

        digits = digits.mul(&digits::DigitsBuf::from(m));
    }
    return digits;
}

pub fn main() -> i64 {
    factorial(100).into_iter().map(|&x| x as u64).sum::<u64>() as i64
}
