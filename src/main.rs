use std::fs;
use std::io;
use std::io::{Error, ErrorKind, Read};

use clap::Clap;

#[derive(Clap)]
struct Opts {
    infile: Option<String>
}

fn read_input(opts: Opts) -> Result<String, Error> {
    match opts.infile {
        Some(filepath) => fs::read_to_string(filepath),
        None => {
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => Ok(buffer),
                Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Could not read from stdin")),
            }
        },
    }
}

fn main() {
    let opts = Opts::parse();
    let input_result = read_input(opts);
    let input = input_result.expect("Could not read file/stdin");
    print!("{}", input);
}
