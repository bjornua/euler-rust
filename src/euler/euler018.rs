use utils::triangles;

const NUMBERS: &'static [u64] =
    &[75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34, 88, 02,
      77, 73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40, 80, 70, 33, 41,
      48, 72, 33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 70, 11, 33,
      28, 77, 73, 17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 63,
      66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 04, 62, 98, 27, 23, 09, 70, 98, 73, 93,
      38, 53, 60, 04, 23];

fn rowsize(n: usize) -> usize {
    let n = n as u64;
    triangles().zip(1..).take_while(|&(t, _)| t <= n).last().unwrap().1
}
fn is_rowstart(n: usize) -> bool {
    let n = n as u64;
    triangles().skip_while(|&x| x < n).next().unwrap() == n
}
fn upleft(n: usize) -> Option<usize> {
    let size = rowsize(n);
    if size > n {
        return None
    }
    return Some(n - size)
}

fn left(n: usize) -> Option<usize> {
    if n < 1 {
        return None
    }
    return Some(n - 1)
}

pub fn maximum_path_sum(numbers: &[u64]) -> u64 {
    let mut numbers = numbers.to_vec();
    let mut i = numbers.len();

    while let Some(n) = left(i) {
        i = n;
        if is_rowstart(n) { continue };

        let parent = upleft(i).unwrap();

        let a = numbers[left(i).unwrap()];
        let b = numbers[i];
        let c = numbers[parent];

        use std::cmp::max;
        numbers[parent] = max(a, b) + c;
    }

    numbers[0]
}

pub fn main() -> i64 {
    maximum_path_sum(NUMBERS) as i64
}
