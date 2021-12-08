use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str, debug: bool) -> String {
    let mut parts = input.split("\n\n");
    let numbers = numbers(&mut parts);

    let (c, bingo) = parts
        .map(Bingo::from)
        .map(|mut b| (b.feed(&numbers), b))
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .and_then(|(c, b)| c.map(|c| (c, b)))
        .unwrap();

    result(bingo, numbers[c], debug).to_string()
}

fn second(input: &str, debug: bool) -> String {
    let mut parts = input.split("\n\n");
    let numbers = numbers(&mut parts);

    let (c, bingo) = parts
        .map(Bingo::from)
        .map(|mut b| (b.feed(&numbers), b))
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .and_then(|(c, b)| c.map(|c| (c, b)))
        .unwrap();

    result(bingo, numbers[c], debug).to_string()
}

fn numbers(numbers: &mut std::str::Split<&str>) -> Vec<i64> {
    numbers
        .next()
        .unwrap()
        .split(',')
        .map(i64::from_str)
        .map(Result::unwrap)
        .collect()
}

fn result(bingo: Bingo, last_called: i64, debug: bool) -> i64 {
    let unmarked = bingo.unmarked();
    if debug {
        println!("Bingo:\n{}", &bingo);
        println!("Input numbers: {:?}", unmarked);
        println!("Unmarked numbers: {:?}", unmarked);
    }

    unmarked.iter().sum::<i64>() * last_called
}

struct Bingo {
    field: Vec<Vec<i64>>,
    found: [[bool; 5]; 5],
}

impl std::fmt::Display for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (y, row) in self.field.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                let prefix = match self.found[x][y] {
                    true => "x",
                    false => " ",
                };
                if value < &10 {
                    print!(" {}{} ", prefix, value);
                } else {
                    print!("{}{} ", prefix, value);
                }
            }
            println!();
        }

        Ok(())
    }
}

impl Bingo {
    /// Returns the amount of numbers that were consumed
    fn feed(&mut self, nums: &[i64]) -> Option<usize> {
        for (i, num) in nums.iter().enumerate() {
            if let Some((x, y)) = self.field.iter().enumerate().find_map(|(y, r)| {
                r.iter().enumerate().find_map(|(x, v)| {
                    if v == num { Some((x, v)) } else { None }.map(|(x, _)| (x, y))
                })
            }) {
                self.found[y][x] = true;
                if self.check_row(y) || self.check_column(x) {
                    return Some(i);
                }
            }
        }

        None
    }

    fn bingo(&self) -> bool {
        (0..5).into_iter().any(|r| self.check_row(r))
            || (0..5).into_iter().any(|c| self.check_column(c))
    }

    fn check_row(&self, row: usize) -> bool {
        self.found[row].iter().all(|&v| v)
    }

    fn check_column(&self, column: usize) -> bool {
        self.found.iter().all(|&r| r[column])
    }

    fn unmarked(&self) -> Vec<i64> {
        self.found
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(_, &f)| !f)
                    .map(move |(x, _)| self.field[y][x])
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl From<&str> for Bingo {
    fn from(from: &str) -> Self {
        Self {
            field: from
                .lines()
                .map(|l| {
                    l.split_whitespace()
                        .map(i64::from_str)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            found: [[false; 5]; 5],
        }
    }
}
