
use utils::{divisors, is_prime};
const NUMBER: u64 = 600_851_475_143;


pub fn main() -> i64 {
    divisors(NUMBER).filter(|&x| is_prime(x)).last().unwrap() as i64
}
