use std::str::FromStr;

pub fn run(input: &str, puzzle: u8) -> String {
    match puzzle {
        1 => first(input),
        2 => second(input),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str) -> String {
    let mut result: u32 = 0;
    let mut previous = input.chars().last().expect("Where are the numbers?");

    for c in input.chars() {
        if c == previous {
            result += u8::from_str(&c.to_string()).unwrap() as u32;
        }

        previous = c;
    }

    result.to_string()
}

fn second(input: &str) -> String {
    let mut result: u32 = 0;
    let cs = input.chars().collect::<Vec<_>>();
    let n = cs.len();
    let half = n/2;

    for (i, c) in cs.iter().enumerate() {
        if c == &cs[(i + half) % n] {
            result += u8::from_str(&c.to_string()).unwrap() as u32;
        }
    }

    result.to_string()
}
