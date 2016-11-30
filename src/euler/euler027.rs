use utils::is_prime;

pub fn main() -> i64 {
    let mut max_n = 0;
    let mut max_pair = (0, 0);
    for a in -999..1000 {
        for b in -1000..1001 {
            // println!("{}", a);
            let mut n = 0;
            loop {
                let prime = n*n + n*a + b;
                if prime < 0 || !is_prime(prime as u64) {
                    break;
                }
                n += 1;
            }
            if max_n < n {
                max_n = n;
                max_pair = (a, b);
            }
        }
    }
    max_pair.0 * max_pair.1
}
