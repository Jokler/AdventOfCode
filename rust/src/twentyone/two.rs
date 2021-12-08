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

    let full_move = input.lines().map(Move::from).sum::<Move>();

    (full_move.forward * full_move.down).to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let (pos, _) =
        input
            .lines()
            .map(Move::from)
            .fold((Position::default(), 0), |(mut pos, mut aim), mov| {
                aim += mov.down;
                if mov.forward > 0 {
                    pos.y += mov.forward;
                    pos.x += aim * mov.forward;
                }

                (pos, aim)
            });

    (pos.x * pos.y).to_string()
}

#[derive(Default)]
struct Move {
    forward: i32,
    down: i32,
}

#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
}

impl From<&str> for Move {
    fn from(input: &str) -> Move {
        let mut split = input.split(" ");
        let (dir, num) = (split.next().unwrap(), split.next().unwrap());
        let num = i32::from_str(num).unwrap();

        let (forward, down) = match dir {
            "forward" => (num, 0),
            "up" => (0, -num),
            "down" => (0, num),
            _ => panic!("not a valid direction: {}", dir),
        };

        Move { forward, down }
    }
}

impl std::ops::Add for Move {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.forward += other.forward;
        self.down += other.down;

        self
    }
}

impl std::iter::Sum for Move {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Move::default(), |a, b| a + b)
    }
}
