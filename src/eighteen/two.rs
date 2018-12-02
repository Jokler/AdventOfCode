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

    let lines = input
        .split_whitespace()
        .tuple_combinations::<(&str, &str)>()
        .map(|(a, b)| (a, b.chars().collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let mut result = String::new();

    for (a, b) in lines {
        let mut diff = 0;
        for (i, c) in a.chars().enumerate() {
            if c != b[i] {
                diff += 1;
                if diff > 1 {
                    break;
                }
            }
        }

        if diff == 1 {
            for (i, c) in a.chars().enumerate() {
                if c == b[i] {
                    result.push(c);
                }
            }

            break;
        }
    }

    result
}
