use utils::divisors;
use std::collections::HashSet;

fn divsum(n: u64) -> u64 {
    divisors(n).filter(|&d| d != n).sum()
}

fn is_abundant(n: u64) -> bool {
    divsum(n) > n
}

pub fn main() -> i64 {
    let abundant_numbers: Vec<_> = (1..28124).filter(|&x| is_abundant(x)).collect();
    let mut forbidden: HashSet<u64> = HashSet::new();

    for (i, &a) in abundant_numbers.iter().enumerate() {
        // println!("{}", i);
        for &b in &abundant_numbers[i..] {
            if a + b > 28124 {
                break;
            }
            forbidden.insert(a + b);
        }
    }
    (1..28124).filter(|n| !forbidden.contains(&n)).sum::<u64>() as i64
}
