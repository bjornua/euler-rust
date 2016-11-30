use std::ops::Add;
use std::mem::replace;
use digits::{DigitsBuf};
struct Fibonacci {
    a: DigitsBuf,
    b: DigitsBuf
}

impl Iterator for Fibonacci {
    type Item = DigitsBuf;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a.add(&self.b);
        let a = replace(&mut self.b, c);
        let ret = replace(&mut self.a, a);

        Some(ret)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { a: DigitsBuf::from(1), b: DigitsBuf::from(1) }
}

pub fn main() -> i64 {
    fibonacci().enumerate().map(|(n, v)| (n+1, v)).filter(|&(_, ref v)| v.len() >= 1_000).next().unwrap().0 as i64
}
