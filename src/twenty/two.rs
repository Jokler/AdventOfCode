use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str, debug: bool) -> String {
    input
        .lines()
        .map(|l| l.split(": "))
        .map(|mut i| (i.next().unwrap(), i.next().unwrap()))
        .map(|(pol, pass)| is_valid(pol, pass, true, debug))
        .filter(|&v| v)
        .count()
        .to_string()
}

fn is_valid(policy: &str, password: &str, old: bool, debug: bool) -> bool {
    let end = policy.len() - 2;
    let bounds = &policy[..end];
    let letter = policy.chars().last().unwrap();

    if debug {
        dbg!(bounds, letter, password);
    }

    let mut iter = bounds.split('-');
    let num1 = iter.next().unwrap();
    let num2 = iter.next().unwrap();
    let num1 = i32::from_str(num1).unwrap();
    let num2 = i32::from_str(num2).unwrap();

    if old {
        old_policy(num1, num2, letter, password)
    } else {
        new_policy(num1, num2, letter, password)
    }
}

fn second(input: &str, debug: bool) -> String {
    input
        .lines()
        .map(|l| l.split(": "))
        .map(|mut i| (i.next().unwrap(), i.next().unwrap()))
        .map(|(pol, pass)| is_valid(pol, pass, false, debug))
        .filter(|&v| v)
        .count()
        .to_string()
}

fn old_policy(lower: i32, upper: i32, letter: char, password: &str) -> bool {
    let count = password.chars().filter(|c| c == letter).count();

    count >= lower && count <= upper
}

fn new_policy(first: i32, second: i32, letter: char, password: &str) -> bool {
    let mut first_valid = false;
    for (i, c) in password.chars().enumerate() {
        let i = i as i32 + 1;
        if i == first {
            if c == letter {
                first_valid = true;
            }
        } else if i == second {
            if first_valid {
                return letter != c;
            } else {
                return letter == c;
            }
        }
    }

    first_valid
}
