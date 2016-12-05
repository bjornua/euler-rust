extern crate primal;
mod euler;
mod utils;
mod digits;

use std::env;
use std::time::{Instant, Duration};

fn duration_to_ms(d: Duration) -> u64 {
    d.as_secs() * 1000 + ((d.subsec_nanos() / 1_000_000)  as u64)
}

fn time_it<T, F: FnOnce() -> T>(f: F) -> (Duration, T) {
    let begin = Instant::now();
    let res = f();
    let elapsed = begin.elapsed();
    return (elapsed, res)
}

fn run_euler(n: u64) -> Option<()> {
    for &(num, func) in EULERS {
        if num == n {
            println!("Problem {n: >3}: https://projecteuler.net/problem={n}", n = n);
            let (time, result) =  time_it(|| func());
            let time_s = duration_to_ms(time) / 1000;
            let time_ms = duration_to_ms(time) % 1000;
            println!("ANSWER  {n: >3}: {time_s}.{time_ms:04}s - {result}", result=result, time_s=time_s, time_ms=time_ms, n=n);
            return Some(());
        }
    }
    return None;
}

fn main() {
    let (_, euler_n): (String, Option<u64>) = {
        let mut args = env::args();
        let program_name = args.next().unwrap();

        let euler_n = args.next().and_then(|x| x.parse::<u64>().ok());
        if args.next().is_some() {
            (program_name, None)
        } else {
            (program_name, euler_n)
        }
    };

    match euler_n.and_then(|n| run_euler(n)) {
        Some(_) => return (),
        None => {
            for &(n, _) in EULERS {
                run_euler(n);
                // println!("Usage: {} {}", program_name, n);
            }
        }
    }
}
const EULERS: &'static [(u64, fn() -> i64)] = &[
    (1, euler::euler001::main),
    (2, euler::euler002::main),
    (3, euler::euler003::main),
    (4, euler::euler004::main),
    (5, euler::euler005::main),
    (6, euler::euler006::main),
    (7, euler::euler007::main),
    (8, euler::euler008::main),
    (9, euler::euler009::main),
    (10, euler::euler010::main),
    (11, euler::euler011::main),
    (12, euler::euler012::main),
    (13, euler::euler013::main),
    (14, euler::euler014::main),
    (15, euler::euler015::main),
    (16, euler::euler016::main),
    (17, euler::euler017::main),
    (18, euler::euler018::main),
    (19, euler::euler019::main),
    (20, euler::euler020::main),
    (21, euler::euler021::main),
    (22, euler::euler022::main),
    (23, euler::euler023::main),
    (24, euler::euler024::main),
    (25, euler::euler025::main),
    (26, euler::euler026::main),
    (27, euler::euler027::main),
    (28, euler::euler028::main),
    (67, euler::euler067::main),
];
