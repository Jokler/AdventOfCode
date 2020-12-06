use std::path::Path;
use structopt::StructOpt; // trait
use structopt_derive::StructOpt;

mod eighteen;
mod nineteen;
mod seventeen;
mod twenty;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "adventofcode",
    about = "Advent of Code in Rust",
    author = "Jokler"
)]
struct Opt {
    #[structopt(help = "Year to run")]
    year: u16,
    #[structopt(help = "Day to run")]
    day: u8,
    #[structopt(help = "First or second")]
    puzzle: u8,
    #[structopt(help = "Input")]
    input: Option<String>,
    #[structopt(short = "i", long = "input", help = "Input")]
    input_file: Option<String>,
    #[structopt(
        short = "o",
        long = "output",
        help = "Output file, stdout if not present"
    )]
    output_file: Option<String>,
    #[structopt(short = "d", long = "debug", help = "Print debug info on some puzzles")]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    let input = match opt.input {
        Some(v) => v,
        None => match opt.input_file {
            Some(v) => read_file(v),
            None => panic!("No input specified!"),
        },
    };

    let output = match opt.year {
        2017 => match opt.day {
            1 => seventeen::one::run(&input, opt.puzzle, opt.debug),
            2 => seventeen::two::run(&input, opt.puzzle, opt.debug),
            3 => seventeen::three::run(&input, opt.puzzle, opt.debug),
            4 => seventeen::four::run(&input, opt.puzzle, opt.debug),
            5 => seventeen::five::run(&input, opt.puzzle, opt.debug),
            6 => seventeen::six::run(&input, opt.puzzle, opt.debug),
            7 => seventeen::seven::run(&input, opt.puzzle, opt.debug),
            8 => seventeen::eight::run(&input, opt.puzzle, opt.debug),
            9 => seventeen::nine::run(&input, opt.puzzle, opt.debug),
            17 => seventeen::seventeen::run(&input, opt.puzzle, opt.debug),
            _ => String::from(format!("There is no code for day {}.", opt.day)),
        },
        2018 => match opt.day {
            1 => eighteen::one::run(&input, opt.puzzle, opt.debug),
            2 => eighteen::two::run(&input, opt.puzzle, opt.debug),
            _ => String::from(format!("There is no code for day {}.", opt.day)),
        },
        2019 => match opt.day {
            1 => nineteen::one::run(&input, opt.puzzle, opt.debug),
            _ => String::from(format!("There is no code for day {}.", opt.day)),
        },
        2020 => match opt.day {
            1 => twenty::one::run(&input, opt.puzzle, opt.debug),
            2 => twenty::two::run(&input, opt.puzzle, opt.debug),
            3 => twenty::three::run(&input, opt.puzzle, opt.debug),
            4 => twenty::four::run(&input, opt.puzzle, opt.debug),
            5 => twenty::five::run(&input, opt.puzzle, opt.debug),
            6 => twenty::six::run(&input, opt.puzzle, opt.debug),
            _ => String::from(format!("There is no code for day {}.", opt.day)),
        },
        _ => String::from(format!("There is no code for year {}.", opt.year)),
    };

    match opt.output_file {
        Some(path) => write_file(path, output),
        None => println!("{}", output),
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut f = File::open(path).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Failed to read file");

    String::from(contents.trim())
}

fn write_file<P: AsRef<Path>>(path: P, output: String) {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output.into_bytes())
        .expect("Failed to write to file");
}
