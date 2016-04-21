use regex::Regex;
use regex::RegexSet;

#[derive(Debug)]
pub struct Renamer {
    set: RegexSet,
    grouped: Regex,
    target: Vec<String>
}

impl Renamer {
    fn new(source_pattern: String, target_pattern: String, delim: char) -> Renamer {
        /*
         * TODO: actually implement
         */
        Renamer {
            set: RegexSet::new(&[""]).unwrap(),
            grouped: Regex::new("").unwrap(),
            target: Vec::new()
        }
    }
}

#[derive(Debug)]
struct Decomp {
    left: String,
    middle: Vec<String>,
    right: String
}

#[derive(Debug)]
struct DecompErr {
    message: String
}

fn decompose_source(source_pattern: String, delim: char) -> Result<Decomp, DecompErr> {
    // TODO: actually implement
    Ok(Decomp {
        left: String::new(),
        middle: Vec::new(),
        right: String::new()
    })
}

fn decompose_target(target_pattern: String, delim: char) -> Result<Decomp, DecompErr> {
    // TODO: actually implement
    Ok(Decomp {
        left: String::new(),
        middle: Vec::new(),
        right: String::new()
    })
}

#[cfg(test)]
mod decompose_test {
    use super::{decompose_source, decompose_target, Decomp, DecompErr};

    parameterized! {
        test_decompose_source;

        ds01: r"" =>
             Ok(("", vec![], ""))
        ds02: r"#" =>
             Err(())
        ds03: r"a" =>
             Ok(("", vec!["a"], ""))
        ds04: r"abcde" =>
             Ok(("", vec!["abcde"], ""))
        ds05: r"ab\#cde" =>
             Ok(("", vec!["ab#cde"], ""))
        ds06: r"(?P<test>.*[a-zA-z]+)\d+" =>
             Ok(("", vec![r"(?P<test>.*[a-zA-z]+)\d+"], ""))
    }

    fn test_decompose_target(param_source_pattern: &'static str, param_expected: Result<(&'static str, Vec<&'static str>, &'static str), ()>) { 
        _test_decompose(false, param_source_pattern, param_expected)
    }

    fn test_decompose_source(param_source_pattern: &'static str, param_expected: Result<(&'static str, Vec<&'static str>, &'static str), ()>) { 
        _test_decompose(true, param_source_pattern, param_expected)
    }

    fn _test_decompose(is_source: bool, param_source_pattern: &'static str, param_expected: Result<(&'static str, Vec<&'static str>, &'static str), ()>) {
        let delim = '#';
        let source_pattern = param_source_pattern.to_owned();
        let result = if is_source {
            super::decompose_source(source_pattern, delim)
        } else {
            super::decompose_target(source_pattern, delim)
        };

        match param_expected {
            Ok((e_left, e_middle, e_right)) => {
                assert!(result.is_ok());
                let super::Decomp {
                    left: r_left,
                    middle: r_middle,
                    right: r_right
                } = result.unwrap();
                assert_eq!(e_left, r_left);
                assert_eq!(e_right, r_right);
                assert_eq!(e_middle, r_middle);
            },
            Err(_) => {
                assert!(result.is_err());
            }
        }
    }
}

