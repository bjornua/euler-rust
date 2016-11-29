pub fn main() -> u64 {
    let mut a = 0;
    while a < 1000 {
        let mut b = a + 1;

        while b < 1000 - a {
            let csquare = a * a + b * b;
            let c = (csquare as f64).sqrt() as u64;

            if c*c == csquare {
                if b < c && a + b + c == 1000 {
                    return a * b * c
                }
            }
            b += 1;
        }
        a += 1;
    }
    panic!("Not found");
}
