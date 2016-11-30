pub fn main() -> i64 {
    let x: u64 = (0..101).into_iter().map(|x| x*x).sum();
    let y: u64 = (0..101).into_iter().sum();

    (y*y - x) as i64
}
