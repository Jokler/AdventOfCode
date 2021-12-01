use itertools::Itertools;
use std::collections::HashMap;

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

    let mut twos = 0;
    let mut threes = 0;

    for line in input.split_whitespace() {
        let mut char_counts = HashMap::new();

        for c in line.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut two = false;
        let mut three = false;
        for (_, count) in char_counts {
            match count {
                2 => two = true,
                3 => three = true,
                _ => (),
            }
        }
        if two {
            twos += 1;
        }

        if three {
            threes += 1;
        }
    }

    (twos * threes).to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    input
        .split_whitespace()
        .tuple_combinations::<(_, _)>()
        .find(|(a, b)| is_solution(a, b))
        .map(|(first, second)| {
            first
                .chars()
                .zip(second.chars())
                .filter(|(a, b)| a == b)
                .map(|(c, _)| c)
                .collect::<String>()
        })
        .unwrap()
}

fn is_solution(first: &str, second: &str) -> bool {
    let mut found_wrong = false;
    for (a, b) in first.chars().zip(second.chars()) {
        if a != b {
            if found_wrong {
                return false;
            }
            found_wrong = true;
        }
    }
    true
}
