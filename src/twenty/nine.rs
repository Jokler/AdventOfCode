use itertools::Itertools;
use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn find_invalid(nums: &[u64]) -> Option<u64> {
    for (i, num) in nums.iter().enumerate().skip(25) {
        let (start, end) = (i - 25, i);
        if !nums[start..end]
            .iter()
            .tuple_combinations()
            .any(|(x, y)| x + y == *num)
        {
            return Some(*num);
        }
    }

    None
}

fn first(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let nums = input
        .lines()
        .map(|l| u64::from_str(l).unwrap())
        .collect_vec();

    find_invalid(&nums).unwrap().to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let nums = input
        .lines()
        .map(|l| u64::from_str(l).unwrap())
        .collect_vec();

    let invalid = find_invalid(&nums).unwrap();

    for window_len in 2..=nums.len() {
        let last_window = nums.len() - window_len;
        for window_start in 0..last_window {
            let window_end = window_start + window_len;
            let window = &nums[window_start..window_end];

            if window.iter().sum::<u64>() == invalid {
                let (min, max) = window.iter().minmax().into_option().unwrap();

                return (min + max).to_string();
            }
        }
    }

    String::from("Found no solution!")
}
