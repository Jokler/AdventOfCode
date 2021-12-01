use itertools::Itertools;
use std::str::FromStr;

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

    input
        .lines()
        .map(|n| i32::from_str(n).unwrap())
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .unwrap()
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    input
        .lines()
        .map(|n| i32::from_str(n).unwrap())
        .tuple_combinations::<(_, _, _)>()
        .find(|(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
        .to_string()
}
