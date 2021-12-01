use std::convert::TryFrom;
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

    let mut machine = Machine::from_str(input);
    let mut known = Vec::new();

    loop {
        if known.contains(&machine.current) {
            break;
        }
        known.push(machine.current);

        machine.step();
    }

    machine.accumulator.to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }

    let machine = Machine::from_str(input);

    for i in 0..machine.instructions.len() {
        let change = match machine.instructions[i] {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(x) => Instruction::Nop(x as i32),
            Instruction::Nop(x) => Instruction::Jmp(x as usize),
        };

        let mut known = Vec::new();
        let mut machine = machine.clone();
        machine.instructions[i] = change;

        loop {
            if known.contains(&machine.current) {
                break;
            }
            known.push(machine.current);

            if machine.step() == Status::Finished {
                return machine.accumulator.to_string();
            }
        }
    }

    String::from("Found no solution")
}

#[derive(Clone)]
struct Machine {
    instructions: Vec<Instruction>,
    current: usize,
    accumulator: i32,
}

#[derive(PartialEq, Eq)]
enum Status {
    Continue,
    Finished,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let instructions = input
            .lines()
            .map(|l| Instruction::try_from(l))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        Self {
            instructions,
            current: 0,
            accumulator: 0,
        }
    }

    fn step(&mut self) -> Status {
        let instruction = match self.instructions.get(self.current) {
            Some(i) => i,
            None => return Status::Finished,
        };
        match instruction {
            Instruction::Acc(x) => self.accumulator += x,
            Instruction::Jmp(x) => {
                self.current += x;
                return Status::Continue;
            }
            Instruction::Nop(_) => (),
        }
        self.current += 1;

        Status::Continue
    }
}

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(usize),
    Nop(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let op = &line[..3];
        let arg = i32::from_str(&line[4..]).map_err(|_| ())?;

        match op {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg as usize)),
            "nop" => Ok(Instruction::Nop(arg)),
            _ => Err(()),
        }
    }
}
