struct PrimeFact(u64);
impl Iterator for PrimeFact {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let PrimeFact(ref mut n) = *self;
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                *n /= i;
                return Some(i)
            }
            i += 1;
        }
        if *n > 1 {
            let res = *n;
            *n = 1;
            Some(res)
        } else {
            None
        }
    }
}

struct Factorial(u64);
impl Iterator for Factorial {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let Factorial(ref mut n) = *self;
        if *n == 0 {
            return None
        };
        *n -= 1;
        return Some(*n + 1);
    }
}

fn reduce_fractions<P: Iterator<Item=u64>, Q: Iterator<Item=u64>>(p: P, q: Q) -> (Vec<u64>, Vec<u64>) {
    let mut ps: Vec<_> = p.flat_map(|n| PrimeFact(n)).collect();
    ps.sort();

    let mut qs: Vec<_> = q.flat_map(|n| PrimeFact(n)).collect();
    qs.sort();
    
    let mut new_q: Vec<u64> = Vec::new();
    let mut new_p: Vec<u64> = Vec::new();

    let mut i = 0;
    for p in ps {
        if let Some(&q) = qs.get(i) {
            if q == p {
                i += 1;
            } else if q > p {
                new_p.push(p);
            } else if q < p {
                i += 1;
                new_q.push(q);
            }
        } else {
            new_p.push(p);
        }
    }
    new_q.extend(&qs[i..]);

    return (new_p, new_q)
}

fn permutations(m: &[u64]) -> u64 {
    let p = Factorial(m.into_iter().sum());
    let q = m.into_iter().flat_map(|&n| Factorial(n));

    let (p, q) = reduce_fractions(p, q);

    p.into_iter().product::<u64>() / q.into_iter().product::<u64>()
}

pub fn main() -> i64 {
    permutations(&[20, 20]) as i64
}
