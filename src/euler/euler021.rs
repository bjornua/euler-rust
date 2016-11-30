use utils::divisors;

fn d(n: u64) -> u64 {
    divisors(n).into_iter().filter(|&x| x < n).sum()
}

pub fn main() -> i64 {
    (1..10_000).filter_map(|n| {
        let a = d(n);
        let b = d(a);

        if b == n && a != n {
            return Some(n)
        }
        None
    }).sum::<u64>() as i64
}
