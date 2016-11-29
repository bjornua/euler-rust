use utils::is_prime;

pub fn main() -> u64 {
    let l: Vec<u64> = (0..2_000_000).into_iter().filter(|&x| is_prime(x)).collect();
    l.iter().sum()
}
