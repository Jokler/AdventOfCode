extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use std::path::Path;

#[derive(StructOpt, Debug)]
#[structopt(name = "adventofcode", about = "Advent of Code in Rust", author = "Jokler")]
struct Opt {
    #[structopt(help = "Day to run")]
    day: u8,
    #[structopt(help = "First or second")]
    puzzle: u8,
    #[structopt(help = "Input")]
    input: Option<String>,
    #[structopt(short = "i", long = "input", help = "Input")]
    input_file: Option<String>,
    #[structopt(short = "o", long = "output", help = "Output file, stdout if not present")]
    output_file: Option<String>,
}

fn main() {
    let opt = Opt::from_args();

    let input = match opt.input {
        Some(v) => v,
        None => match opt.input_file {
            Some(v) => read_file(v),
            None => panic!("No input specified!"),
        }
    };

    let output = match opt.day {
        _ => String::from(format!("There is no code for day {}.", opt.day))
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

    let mut file = File::create(path).expect("Failed to create to file");

    match file.write_all(&output.into_bytes()) {
        Ok(_) => {},
        Err(e) => println!("Failed to write to file ({})", e),
    }
}
