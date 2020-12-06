use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

static VALID_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn first(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let is_valid = |v: &&str| VALID_FIELDS.contains(&&v[..3]);

    input
        .split("\n\n")
        .map(|p| p.split_whitespace().filter(is_valid).count() == 7)
        .count()
        .to_string()
}

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "twenty/four.pest"]
pub struct PassParser;

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut count = 0;

    'outer: for passport in input.split("\n\n") {
        let parsed = PassParser::parse(Rule::advent, passport);

        if let Ok(parsed) = parsed {
            for field in parsed.into_iter().next().unwrap().into_inner() {
                let is_valid = match field.as_rule() {
                    Rule::byr => valid_byr(field.into_inner().as_str()),
                    Rule::iyr => valid_iyr(field.into_inner().as_str()),
                    Rule::eyr => valid_eyr(field.into_inner().as_str()),
                    Rule::hgt => valid_hgt(field.into_inner().next().unwrap()),
                    Rule::EOI => true,
                    Rule::hex_char
                    | Rule::hcl
                    | Rule::ecl
                    | Rule::pid
                    | Rule::cid
                    | Rule::inches
                    | Rule::cm
                    | Rule::year
                    | Rule::advent
                    | Rule::WHITESPACE
                    | Rule::COMMENT => unreachable!(),
                };

                if !is_valid {
                    continue 'outer;
                }
            }
            count += 1;
        }
    }

    count.to_string()
}

fn valid_byr(year: &str) -> bool {
    let year = u32::from_str(year).unwrap();

    year >= 1920 && year <= 2002
}

fn valid_iyr(year: &str) -> bool {
    let year = u32::from_str(year).unwrap();

    year >= 2010 && year <= 2020
}

fn valid_eyr(year: &str) -> bool {
    let year = u32::from_str(year).unwrap();

    year >= 2020 && year <= 2030
}

fn valid_hgt(height: pest::iterators::Pair<Rule>) -> bool {
    let rule = height.as_rule();
    let height = height.as_str();
    let height = u32::from_str(&height[..height.len() - 2]).unwrap();
    match rule {
        Rule::cm => height >= 150 && height <= 193,
        Rule::inches => height >= 59 && height <= 76,
        _ => unreachable!(),
    }
}
