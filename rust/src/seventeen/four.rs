use itertools::Itertools;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let result: u32 = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .tuple_combinations()
                .any(|(a, b)| a == b)
        })
        .fold(0, |acc, invalid| if invalid { acc } else { acc + 1 });

    result.to_string()
}

fn sort(s: &str) -> String {
    use std::iter::FromIterator;

    let mut v = s.chars().collect::<Vec<_>>();
    v.sort_unstable();

    String::from_iter(v)
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let result: u32 = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .tuple_combinations()
                .any(|(a, b)| sort(a) == sort(b))
        })
        .fold(0, |acc, invalid| if invalid { acc } else { acc + 1 });

    result.to_string()
}
