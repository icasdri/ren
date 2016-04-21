use regex::Regex;
use regex::RegexSet;

#[derive(Debug)]
pub struct Renamer {
    set: RegexSet,
    target: Vec<String>
}

impl Renamer {
    fn new(source_pattern: String, target_pattern: String, delim: char) -> Renamer {
        /*
         * TODO: actually implement
         */
        Renamer {
            set: RegexSet::new(&[""]).unwrap(),
            target: Vec::new()
        }
    }
}

fn decompose_source(source_pattern: String) -> Vec<String> {
    Vec::new() // TODO: actually implement
}

fn decompose_target(target_pattern: String) -> Vec<String> {
    Vec::new() // TODO: actually implement
}

#[cfg(test)]
mod test {
    use super::*;
    
    parameterized_tests! {
        super::decompose_source;

        // TODO: actual tests -- these are just contrived examples to demonstrate the macro
        // a1: String::from("Hi") => Vec::<String>::new(),
        // a2: String::from("Hello") => Vec::<String>::new()
    }
}
