
use utils::{triangles, divisors};
pub fn main() -> u64 {
    triangles().filter(|&n| divisors(n).count() > 500).next().unwrap()
}
