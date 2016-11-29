use utils::is_prime;


pub fn main() -> u64 {
    let mut count = 0;
    let mut x = 2;
    loop {
        if is_prime(x) {
            count +=1;
            if count == 10001 {
                return x;
            }
        }
        x += 1;
    }
}
