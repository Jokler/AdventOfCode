use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str, debug: bool) -> String {
    let mut result: u32 = 0;

    let mut nums = input
        .split_whitespace()
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut previous = Vec::new();
    loop {
        if previous.iter().any(|v: &Vec<u32>| v == &nums) {
            break;
        }
        previous.push(nums.clone());

        let (mut curr, max) = nums
            .iter()
            .map(|n| *n)
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)))
            .expect("Numbers?");

        if debug {
            println!("vec: {:?}", nums);
        }

        nums[curr as usize] = 0;
        for _ in 0..max {
            curr = (curr + 1) % nums.len();
            nums[curr as usize] += 1;
        }

        result += 1;
    }

    result.to_string()
}

fn second(input: &str, debug: bool) -> String {
    let mut result: u32 = 0;

    let mut nums = input
        .split_whitespace()
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut previous = Vec::new();
    let mut in_loop = false;
    loop {
        if previous.iter().any(|v: &Vec<u32>| v == &nums) {
            if in_loop {
                break;
            } else {
                previous.clear();
                in_loop = true;
                if debug {
                    println!("Starting loop");
                }
            }
        }
        previous.push(nums.clone());

        let (mut curr, max) = nums
            .iter()
            .map(|n| *n)
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)))
            .expect("Numbers?");

        if debug {
            println!("vec: {:?}", nums);
        }

        nums[curr as usize] = 0;
        for _ in 0..max {
            curr = (curr + 1) % nums.len();
            nums[curr as usize] += 1;
        }

        if in_loop {
            result += 1;
        }
    }

    result.to_string()
}
