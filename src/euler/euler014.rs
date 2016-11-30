struct CollatzSequence(u64);
impl CollatzSequence {
    fn new(n: u64) -> Self {
        CollatzSequence(n)
    }
}
impl Iterator for CollatzSequence {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let CollatzSequence(ref mut n) = *self;
        if *n == 0 {
            return None;
        }
        if *n == 1 {
            *n = 0;
            return Some(1);
        }
        let res = *n;
        *n = match *n % 2 {
            0 => *n / 2,
            _ => *n * 3 + 1,
        };
        Some(res)
    }
}

pub fn main() -> i64 {
    let (n, _) = (1..1_000_000)
        .map(|n| (n, CollatzSequence::new(n).count()))
        .max_by_key(|&(_, count)| count)
        .unwrap();
    n as i64
}
