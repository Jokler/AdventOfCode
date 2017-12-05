use std::str::FromStr;

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

    let mut nums = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| i32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut cur: i32 = 0;
    let mut offset = 0;
    loop {
        if 0 > cur || cur as usize >= nums.len() {
            break;
        }
        let old = offset;
        offset = nums[cur as usize];
        nums[cur as usize] = offset + 1;
        cur += offset;
        result += 1;
    }

    result.to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut result: u32 = 0;

    let mut nums = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| i32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut cur: i32 = 0;
    loop {
        if 0 > cur || cur as usize >= nums.len() {
            break;
        }
        let offset = nums[cur as usize];

        if offset > 2 {
            nums[cur as usize] = offset - 1;
        } else {
            nums[cur as usize] = offset + 1;
        }
        cur += offset;
        result += 1;
    }

    result.to_string()
}
