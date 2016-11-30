const MAX: u64 = 4_000_000;



pub fn main() -> i64 {
    let mut a: u64 = 1;
    let mut b: u64 = 2;

    let mut sum = 2;

    loop {
        let c = a + b;

        if c >= MAX {
            return sum as i64
        }
        if c % 2 == 0 {
            sum += c;
        }

        a = b;
        b = c;
    }


}
