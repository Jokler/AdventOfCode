use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: &str, puzzle: u8, debug: bool) -> String {
    match puzzle {
        1 => first(input, debug),
        2 => second(input, debug),
        _ => String::from("There are only 2 puzzles per day"),
    }
}

struct Operation<'a> {
    calc: &'a str,
    expr: &'a str,
}

impl<'a> Operation<'a> {
    pub fn execute(&self, registers: &mut HashMap<&'a str, i32>) {
        let mut split = self.calc.split_whitespace();
        let register = split.next().unwrap();
        let op = split.next().unwrap();
        let num = i32::from_str(split.next().unwrap()).unwrap();

        if self.is_expr_true(registers) {
            let reg = registers.entry(register).or_insert(0);
            match op {
                "inc" => *reg += num,
                "dec" => *reg -= num,
                _ => panic!("Unknown Op"),
            }
        }
    }

    fn is_expr_true(&self, registers: &mut HashMap<&'a str, i32>) -> bool {
        let mut split = self.expr.split_whitespace();
        let reg = split.next().unwrap();
        let op = split.next().unwrap();
        let num = i32::from_str(split.next().unwrap()).unwrap();

        let reg = *registers.entry(reg).or_insert(0);
        match op {
            ">" => reg > num,
            "<" => reg < num,
            ">=" => reg >= num,
            "<=" => reg <= num,
            "==" => reg == num,
            "!=" => reg != num,
            _ => panic!("Unknown comparison"),
        }
    }
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(input: &'a str) -> Self {
        let mut split = input.split(" if ");
        Operation {
            calc: split.next().expect("Empty line"),
            expr: split.next().expect("Missing if"),
        }
    }
}

fn first(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut registers = HashMap::<&str, i32>::new();

    for line in input.split("\n") {
        let op: Operation = line.into();
        op.execute(&mut registers);
    }

    registers
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .1
        .to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let mut registers = HashMap::<&str, i32>::new();
    let mut max = 0;

    for line in input.split("\n") {
        let op: Operation = line.into();
        op.execute(&mut registers);

        max = *registers
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .1
            .max(&max)
    }

    max.to_string()
}
