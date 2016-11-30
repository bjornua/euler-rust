
pub fn is_divisable(i: u64) -> bool {
    if i % 20 == 0 && i % 19 == 0 && i % 18 == 0 && i % 17 == 0 && i % 16 == 0 &&
       i % 15 == 0 && i % 14 == 0 && i % 13 == 0 && i % 12 == 0 &&
       i % 11 == 0 && i % 10 == 0 && i % 9 == 0 && i % 8 == 0 &&
       i % 7 == 0 && i % 6 == 0 &&
       i % 5 == 0 && i % 4 == 0 && i % 3 == 0 && i % 2 == 0 {
        return true;
    }
    return false;

}

pub fn main() -> i64 {
    let mut n = 20;
    while !is_divisable(n) {
        n += 20;
    }
    n as i64
}
