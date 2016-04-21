extern crate argparse;
extern crate regex;

#[macro_use]
#[cfg(test)]
mod testing_utils;

mod renamer;

use std::process;
use argparse::{ArgumentParser, StoreTrue, Store};
use renamer::Renamer;

#[derive(Debug)]
struct ExitWith {
    exit_code: i32,
    message: String,
}

impl ExitWith {
    fn new(exit_code: i32, message: String) -> ExitWith {
        ExitWith {
            exit_code: exit_code,
            message: message
        }
    }
}

fn main() {
    if let Err(e) = encapsulated_main() {
        println!("{}", e.message);
        process::exit(e.exit_code);
    }
}

fn encapsulated_main() -> Result<(), ExitWith> {
    let mut source_pattern = String::with_capacity(30);
    let mut target_pattern = String::with_capacity(20);
    let mut flag_interactive = false;
    let mut flag_execute = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Regex-based batch filename manipulation.");
        parser.refer(&mut source_pattern)
            .add_argument("source", Store, "regex search pattern for source files")
            .required();
        parser.refer(&mut target_pattern)
            .add_argument("target", Store, "regex target replacement pattern")
            .required();
        parser.refer(&mut flag_interactive)
            .add_option(&["-i", "--interactive"], StoreTrue, "run in interactive mode");
        parser.refer(&mut flag_execute)
            .add_option(&["-e", "--execute"], StoreTrue, "execute directly on filesystem");
        match parser.parse_args() {
            Err(0) => return Ok(()),
            Err(e) => return Err(ExitWith::new(e, String::new())),
            _ => {}
        }
    }

    println!("source_pattern: {}", source_pattern);
    println!("target_pattern: {}", target_pattern);
    println!("flag_interactive: {}", flag_interactive);
    println!("flag_execute: {}", flag_execute);
    Ok(())
}
