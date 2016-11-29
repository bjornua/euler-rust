fn is_palindrome(n: u64) -> bool {
    let mut n = n.clone();
    let mut digits: [u8; 20] = [0; 20];
    let mut cursor = 0;

    loop {
        digits[cursor] = (n % 10) as u8;
        cursor += 1;
        n = n / 10;
        if n == 0 {
            break
        }
    }
    let mut i: usize = 0;
    while i < cursor / 2 {
        if digits[i] != digits[cursor - 1 - i] {
            return false
        }
        i += 1;
    }
    true

}

pub fn main() -> u64 {
    let mut x = 999;
    let mut highest = 0;
    loop {
        let mut y = x;
        loop {
            let result = x*y;
            if result > highest && is_palindrome(result){
                highest = result;
            }
            if y == 100 {
                break;
            }
            y -= 1;
        }

        if x == 100 {
            break
        } else {

        }
        x -= 1;
    }
    return highest
}
