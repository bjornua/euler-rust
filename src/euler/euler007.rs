use utils::is_prime;


pub fn main() -> i64 {
    let mut count = 0;
    let mut x = 2;
    loop {
        if is_prime(x) {
            count +=1;
            if count == 10001 {
                return x as i64
            }
        }
        x += 1
    }
}
