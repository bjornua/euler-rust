fn between(n: u64, a: u64, b: u64) -> bool {
    if n < a {
        return false;
    }
    return n <= b;
}

fn to_string(n: u64) -> String {
    match n {
        0 => "".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        15 => "fifteen".to_string(),
        18 => "eighteen".to_string(),
        14 | 16 | 17 | 19 => to_string(n%10) + "teen",
        n if n < 30 => "twenty".to_string() + &to_string(n%10),
        n if between(n, 30, 39) => "thirty".to_string() + &to_string(n%10),
        n if between(n, 40, 49) => "forty".to_string() + &to_string(n%10),
        n if between(n, 50, 59) => "fifty".to_string() + &to_string(n%10),
        n if between(n, 80, 89) => "eighty".to_string() + &to_string(n%10),
        n if n < 100 => to_string(n/10%10) + "ty" + &to_string(n%10),
        n if n < 1_000 => to_string(n/100%10) + &(match to_string(n%100) {
            ref s if s.len() == 0 => "hundred".to_string(),
            ref s => "hundredand".to_string() + s
        }),
        n if n < 1_000_000 => to_string(n/1000%1000) + &(match to_string(n % 1000) {
            ref s if s.len() == 0 => "thousand".to_string(),
            ref s => "thousandand".to_string() + s
        }),
        _ => panic!("Panic: Out of range {}", n)
    }
}

pub fn main() -> u64 {
    let mut sum = 0;
    for n in 1..1001 {
        sum += to_string(n).len();
    }
    sum as u64
}
