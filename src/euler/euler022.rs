const NAMES: &'static str = include_str!("../data/euler022_names.txt");

enum ParserState {
    Begin,
    Quoted(usize),
    End,
}

use self::ParserState::*;
fn names_parser(input: &str) -> Vec<&str> {
    let mut output: Vec<&str> = Vec::new();
    let mut state = Begin;

    for (i, c) in input.char_indices() {
        state = match (state, c) {
            (Begin, '"') => Quoted(i + '"'.len_utf8()),
            (Quoted(b), '"') => {
                output.push(&input[b..i]);
                End
            }
            (Quoted(b), _) => Quoted(b),
            (End, ',') => Begin,
            (Begin, c) | (End, c) => panic!("Invalid character {:?} at position {}", c, i),
        }
    }
    output
}
fn char_value(c: char) -> u64 {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .filter(|&(_, d)| c == d)
        .map(|(n, _)| (n as u64) + 1)
        .next()
        .expect("Invalid character")
}

pub fn main() -> i64 {
    let mut names = names_parser(NAMES);
    names.sort();
    names.into_iter().map(|name| name.chars().map(char_value).sum::<u64>()).enumerate().map(|(n, s)| ((n as u64) + 1) * s).sum::<u64>() as i64
}
