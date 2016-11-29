pub struct Divisors {
    factor: u64,
    number: u64,
    max: u64,
    alt: u64,
}


pub fn divisors(n: u64) -> Divisors {
    Divisors {
        factor: 0,
        number: n,
        max: ((n as f64).sqrt() as u64),
        alt: 0
    }
}

impl Iterator for Divisors {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let Divisors { ref mut factor, number, ref max, ref mut alt} = *self;
        if *alt != 0 {
            let res = *alt;
            *alt = 0;
            return Some(res);
        }
        while *factor < *max {
            *factor += 1;
            if number % *factor == 0 {
                *alt = number / *factor;
                if factor == alt {
                    *alt = 0;
                }
                return Some(*factor)
            }
        }
        return None
    }
}


pub struct TriangleNumbers(u64, u64);
pub fn triangles() -> TriangleNumbers {
    TriangleNumbers(0, 1)
}

impl Iterator for TriangleNumbers {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let TriangleNumbers(ref mut current, ref mut n) = *self;
        let res = *current;
        *current += *n;
        *n += 1;
        return Some(res)
    }
}


pub fn is_prime(u: u64) -> bool {
     if u < 2 {
         return false;
     }
     divisors(u).skip(2).next().is_none()
}

