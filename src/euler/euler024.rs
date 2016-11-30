struct Permutations<'a, T: 'a> {
    current: Vec<(usize, &'a T)>,
    done: bool,
}

// Yes this is not pretty but it's correct
impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let ret = self.current.iter().map(|&(_, v)| v).collect();
        let indexes = {
            let k = self.current
                .iter()
                .zip(&self.current[1..])
                .enumerate()
                .map(|(i, (&(a, _), &(b, _)))| (i, a, b))
                .filter(|&(_, a, b)| a < b)
                .map(|(i, a, _)| (i, a))
                .last();

            k.and_then(|(k, ak)| {
                self.current
                    .iter()
                    .enumerate()
                    .map(|(i, &(al, _))| (i, al))
                    .filter(|&(i, al)| i > k && ak < al)
                    .map(|(i, _)| i)
                    .last()
                    .map(|i| (k, i))
            })
        };
        if let Some((k, l)) = indexes {
            self.current.swap(k, l);
            self.current[(k + 1)..].reverse();
        } else {
            self.done = true;
        }
        Some(ret)
    }
}
fn permutations<'a, T>(items: &'a [T]) -> Permutations<'a, T> {
    let collection: Vec<(usize, &'a T)> = items.into_iter().enumerate().collect();
    Permutations {
        current: collection,
        done: false,
    }
}


pub fn main() -> i64 {
    const VALUES: &'static [u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let p = permutations(VALUES).nth(999_999).unwrap();

    p.into_iter().rev().enumerate().map(|(n, s)| (10 as u64).pow(n as u32) * (*s as u64)).sum::<u64>() as i64
}
