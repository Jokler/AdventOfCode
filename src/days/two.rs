use std::str::FromStr;
use std::u32;
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

    let mut result: u32 = 0;
    let rows = input.split("\n").collect::<Vec<_>>();

    for row in rows {
        let nums = row.split_whitespace()
            .map(|x| u32::from_str(x).unwrap())
            .collect::<Vec<_>>();

        let min = nums.iter().min().expect("Could not find minimum");
        let max = nums.iter().max().expect("Could not find maximum");

        result += max - min;
    }

    result.to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut result: u32 = 0;
    let rows = input.split("\n").collect::<Vec<_>>();

    for row in rows {
        let nums = row.split_whitespace()
            .map(|x| u32::from_str(x).unwrap())
            .collect::<Vec<_>>();

        result += nums.iter()
            .tuple_combinations()
            .map(|(a, b)| (a.max(b), a.min(b)))
            .filter(|&(a, b)| a % b == 0)
            .map(|(a, b)| a / b)
            .next()
            .expect("Could not find values on every line");
    }

    result.to_string()
}
