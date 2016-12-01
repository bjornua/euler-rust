/*
21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13


1 3 5 7 9 13 17 21 25
0 2 2 2 2 4  4  4  4
*/

const SIZE: u64 = 1001;

pub fn main() -> i64 {
    let mut n = 1;
    let mut spacing = 2;
    let mut diagonal = 0;
    let mut sum = 0;
    while n <= SIZE * SIZE {
        sum = sum + n;

        n = n + spacing;
        if diagonal == 3 {
            diagonal = 0;
            spacing = spacing + 2;
        } else {
            diagonal = diagonal + 1;
        }
    }
    sum as i64
}
