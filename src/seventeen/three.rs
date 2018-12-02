use std::str::FromStr;

use itertools::Itertools;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

fn first(input: &str, debug: bool) -> String {
    let num = i32::from_str(input).expect("Bad Input");

    if num == 1 {
        return format!("{}", 0);
    }

    let root = (num as f32).sqrt().ceil() as i32;
    let width = if root % 2 != 0 { root } else { root + 1 };

    let a = (width as f32 / 2.0).floor() as i32;
    let b = ((num - (2 * a - 1).pow(2)) % (2 * a) - a).abs();

    if debug {
        println!("root: {}, width: {}, a: {}, b: {}", root, width, a, b);
    }

    let result = a + b;

    result.to_string()
}

#[derive(Clone, Debug)]
struct Field {
    x: i32,
    y: i32,
    i: u32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn next_val(v: &Vec<Field>, x: i32, y: i32) -> u32 {
    v.iter()
        .filter(|f| f.x >= x - 1 && f.x <= x + 1)
        .filter(|f| f.y >= y - 1 && f.y <= y + 1)
        .fold(0, |acc, f| acc + f.i)
}

fn second(input: &str, debug: bool) -> String {
    use self::Direction::*;

    let result: u32;
    let num = u32::from_str(input).unwrap();

    let mut spiral = vec![Field { x: 0, y: 0, i: 1 }];

    let mut dir = Right;
    let (mut x, mut y, mut dist) = (0, 0, 0);

    loop {
        if dir == Up {
            y -= 1;

            if y <= -dist {
                dir = Left;
            }

        } else if dir == Down {
            y += 1;

            if y >= dist {
                dir = Right;
            }

        } else if dir == Left {
            x -= 1;

            if x <= -dist {
                dir = Down;
            }

        } else if dir == Right {
            x += 1;

            if x > dist {
                dist += 1;
                dir = Up;
            }
        }

        let i = next_val(&spiral, x, y);
        let field = Field { x: x, y: y, i: i };
        spiral.push(field);

        if i > num {
            if debug {
                print_spiral(&spiral);
                print!("\nResult: ");
            }

            result = i;
            break;
        }
    }

    result.to_string()
}

fn print_spiral(spiral: &Vec<Field>) {
    println!("Debug printing only works well for small spirals");

    let min = spiral
        .iter()
        .fold(0, |result, f| if f.x < result { f.x } else { result })
        .abs();

    for i in -min..(min + 1) {
        let row = spiral
            .iter()
            .cloned()
            .filter(|f| f.y == i)
            .sorted_by(|a, b| Ord::cmp(&a.x, &b.x));

        if row.is_empty() {
            break;
        }

        for _ in 0..(min + row.first().unwrap().x) {
            print!("\t");
        }

        println!("{}", row.iter().map(|f| f.i).join("\t"));
    }
}
