use std::str::FromStr;
use std::convert::TryFrom;
use itertools::Itertools;

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

    let mut memory = input
        .split(',')
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    memory[1] = 12;
    memory[2] = 2;

    run_machine(memory).to_string()
}

fn second(input: &str, debug: bool) -> String {
    if debug {
        println!("No debug output for this puzzle");
    }
    let mut memory = input
        .split(',')
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;
    
    for (noun, verb) in (0..99).tuple_combinations::<(_, _)>() {
        memory[1] = noun;
        memory[2] = verb;

        if run_machine(memory.clone()) == 19690720 {
            result = 100 * noun + verb;
            break;
        }
    }

    result.to_string()
}

fn run_machine(mut memory: Vec<u32>) -> u32 {
    let mut pos = 0;

    loop {
        let opcode = memory[pos];
        let op1 = memory[pos + 1] as usize;
        let op2 = memory[pos + 2] as usize;
        let target = memory[pos + 3] as usize;

        match Opcode::try_from(memory[pos]).unwrap() {
            Opcode::Add => memory[target] = memory[op1] + memory[op2],
            Opcode::Multiply => memory[target] = memory[op1] * memory[op2],
            Opcode::Halt => return memory[0],
        }

        pos += 4;
    }
}

enum Opcode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl TryFrom<u32> for Opcode {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == Opcode::Add as u32 => Ok(Opcode::Add),
            x if x == Opcode::Multiply as u32 => Ok(Opcode::Multiply),
            x if x == Opcode::Halt as u32 => Ok(Opcode::Halt),
            _ => Err(()),
        }
    }
}
