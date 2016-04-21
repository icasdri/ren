use regex::Regex;
use regex::RegexSet;

#[derive(Debug)]
pub struct Renamer {
    set: RegexSet,
    target: Vec<String>
}

impl Renamer {
    fn new(source_pattern: String, target_pattern: String, delim: char) -> Renamer {
        
    }
}

fn decompose_source(source_pattern: String) -> Vec<String> {

}

fn decompose_target(target_pattern: String) -> Vec<String> {

}
