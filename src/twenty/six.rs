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

    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| !c.is_whitespace())
                .unique()
                .count() as u32
        })
        .sum::<u32>()
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(None, |yes: Option<String>, p| {
                    yes.map(|q| q.chars().filter(|&c| p.contains(c)).collect::<String>())
                        .or_else(|| Some(p.to_string()))
                })
                .map(|s| s.len() as u32)
                .unwrap_or(0)
        })
        .sum::<u32>()
        .to_string()
}
