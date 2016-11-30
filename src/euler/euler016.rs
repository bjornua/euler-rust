// 2⁴ =  16
// 2⁵ =  32 = 16 + 16
// 2⁶ =  64 = 32 + 32
// 2⁷ = 128 = 64 + 64


// use digits;
fn add(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(a.len() + 1);
    let mut remainder = 0;
    let mut b = b.into_iter().rev();
    for a in a.into_iter().rev() {
        let b = b.next().cloned().unwrap_or(0);
        let sum = a + b + remainder;
        remainder = sum / 10;
        result.push(sum % 10);
    }
    if remainder != 0 {
        result.push(remainder)
    }
    result.reverse();
    return result
}

pub fn main() -> i64 {
    let mut n = vec![1];
    for _ in 0..1000 {
        n = add(&n, &n);
    }
    n.into_iter().sum::<u64>() as i64
}
