const MAX: u64 = 1000;

pub fn main() -> i64 {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let mut sum: u64 = 0;

    while x < MAX {
        sum += x;
        x += 3;
    }
    while y < MAX {
        if y % 3 != 0 {
            sum += y;
        }
        y += 5;
    }

    sum as i64
}
