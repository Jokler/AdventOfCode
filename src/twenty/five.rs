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
        .lines()
        .map(parse_position)
        .map(seat_id)
        .max()
        .unwrap()
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut sorted_ids = input
        .lines()
        .map(parse_position)
        .map(seat_id)
        .collect::<Vec<_>>();

    sorted_ids.sort();

    let mut prev = 0;
    for id in sorted_ids {
        if id - prev == 2 {
            return (prev + 1).to_string();
        } else {
            prev = id;
        }
    }

    panic!("No seat was found!");
}

fn seat_id(seat: (u32, u32)) -> u32 {
    let (r, c) = seat;

    r * 8 + c
}

fn parse_position(seat: &str) -> (u32, u32) {
    let row_str = &seat[..7].replace("F", "0").replace("B", "1");
    let col_str = &seat[7..].replace("L", "0").replace("R", "1");

    let row = u32::from_str_radix(row_str, 2).unwrap();
    let col = u32::from_str_radix(col_str, 2).unwrap();

    (row, col)
}
