
use utils::{triangles, divisors};
pub fn main() -> i64 {
    triangles().filter(|&n| divisors(n).count() > 500).next().unwrap() as i64
}
