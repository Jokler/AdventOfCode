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

    let lines = input.lines().collect::<Vec<_>>();
    let line_len = lines[0].len();

    let (mut x, mut y) = (0, 0);
    let mut count = 0;

    while y < lines.len() {
        if lines[y].as_bytes()[x] == b'#' {
            count += 1;
        }

        y += 1;
        x = (x + 3) % line_len;
    }

    count.to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let options = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let lines = input.lines().collect::<Vec<_>>();
    let line_len = lines[0].len();

    let mut result: u64 = 1;

    for (x_offset, y_offset) in options {
        let (mut x, mut y) = (0, 0);
        let mut count = 0;

        while y < lines.len() {
            if lines[y].as_bytes()[x] == b'#' {
                count += 1;
            }

            y += y_offset;
            x = (x + x_offset) % line_len;
        }

        result *= count;
    }

    result.to_string()
}
