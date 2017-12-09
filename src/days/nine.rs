
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
    let mut escaped = false;

    for c in input.chars() {
        if garbage {
            if escaped {
                escaped = false;
            } else {
                match c {
                    '!' => escaped = true,
                    '>' => garbage = false,
                    _ => (),
                }
            }
        } else {
            if escaped {
                escaped = false;
            } else {
                match c {
                    '{' => {
                        indent +=1;
                        result += indent;
                    }
                    '}' => indent -= 1,
                    '!' => escaped = true,
                    '<' => garbage = true,
                    _ => (),
                }
            }
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
    let mut escaped = false;

    for c in input.chars() {
        if garbage {
            if escaped {
                escaped = false;
            } else {
                match c {
                    '!' => escaped = true,
                    '>' => garbage = false,
                    _ => result += 1,
                }
            }
        } else {
            if escaped {
                escaped = false;
            } else {
                match c {
                    '!' => escaped = true,
                    '<' => garbage = true,
                    _ => (),
                }
            }
        }
    }

    result.to_string()
}
