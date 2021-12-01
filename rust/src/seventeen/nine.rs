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

    let mut result = 0;
    let mut indent = 0;
    let mut garbage = false;

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '!' => {
                chars.next();
            }
            '>' => garbage = false,
            _ if garbage => (),
            '<' => garbage = true,
            '{' => {
                indent += 1;
                result += indent;
            }
            '}' => indent -= 1,
            _ => (),
        }
    }

    result.to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut result = 0;
    let mut garbage = false;

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '!' => {
                chars.next();
            }
            '>' => garbage = false,
            _ if garbage => result += 1,
            '<' => garbage = true,
            _ => (),
        }
    }

    result.to_string()
}
