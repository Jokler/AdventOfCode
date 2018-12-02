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

    input.split_whitespace().map(|n| i32::from_str(n).unwrap()).sum::<i32>()
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let nums = input
        .split_whitespace()
        .map(|n| i32::from_str(n).unwrap()).collect::<Vec<_>>();

    let mut current = 0;
    let mut previous = Vec::new();

    for num in nums.iter().cycle() {
        current += num;
        if previous.contains(&current) {
            break;
        }
        previous.push(current);
    }

    current.to_string()
}
