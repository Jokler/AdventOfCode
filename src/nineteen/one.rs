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
        .split_whitespace()
        .map(|n| i32::from_str(n).unwrap())
        .map(|i| fuel(i))
        .sum::<i32>()
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    input
        .split_whitespace()
        .map(|n| i32::from_str(n).unwrap())
        .map(|i| fuel(i))
        .map(|i| i + extra_fuel(i, debug))
        .sum::<i32>()
        .to_string()
}

fn fuel(mass: i32) -> i32 {
    (mass / 3 - 2).max(0)
}

fn extra_fuel(mut unfueled_mass: i32, debug: bool) -> i32 {
    let mut result = 0;
    while unfueled_mass > 0 {
        unfueled_mass = fuel(unfueled_mass);
        result += unfueled_mass;

        if debug {
            dbg!(unfueled_mass);
        }
    }

    result
}
