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

    let num = usize::from_str(input).unwrap();
    let mut vec = vec![0];
    let mut current = 0;

    for i in 1..(2017 + 1) {
        let len = vec.len();
        current = (current + num) % len;

        current += 1;
        vec.insert(current, i);
    }

    let len = vec.len();
    vec[(current + 1) % len].to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let num = usize::from_str(input).unwrap();
    let mut current = 0;
    let mut result = 0;

    for i in 1..(50_000_000 + 1) {
        current = (current + num) % i;

        current += 1;

        if current == 1 {
            result = i;
        }
    }

    result.to_string()
}
