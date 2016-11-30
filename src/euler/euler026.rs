
pub fn unit_fraction(q: u64) -> (Vec<u8>, Vec<u8>) {
    let mut decimals = Vec::new();
    let mut remainders = vec![10];

    loop {
        let p = remainders.last().unwrap().clone();

        let fraction = p / q;
        decimals.push(fraction as u8);

        let new_p = (p % q) * 10;
        if let Some((i, _)) = remainders.iter().enumerate().find(|&(_, &old_p)| old_p == new_p) {
            let repeating = decimals.split_off(i);
            // println!("{} {} {}", q, i, repeating.len());
            return (decimals, repeating)
        } else {
            remainders.push(new_p);
        }
    }
}

pub fn main() -> i64 {
    (1..1000).map(|n| (n, unit_fraction(n))).max_by_key(|&(_, (_, ref repeating))| {
        repeating.len()
    }).unwrap().0 as i64
}
