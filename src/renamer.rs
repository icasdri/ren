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
    let escape_seq = {
        let mut x = String::new();
        x.push('\\');
        x.push(delim);
        x
    };
    let mut store: Vec<String> = Vec::new();

    for s in source_pattern.split(escape_seq.as_str()) {
        let mut iter = s.split(delim);
        if let Some(mut last) = store.pop() {
            if let Some(first) = iter.next() {
                last.push_str(first);
                store.push(last);
            }
        }
        for x in iter {
            store.push(x.to_owned());
        }
    }
    
    match store.len() {
        0 | 2 => Err(DecompErr { message: String::from("delim syntax error") }),
        1 => Ok(Decomp {
            left: String::new(),
            middle: store,
            right: String::new()
        }),
        _ => {
            let left = store.remove(0);
            let right = store.pop().unwrap();
            Ok(Decomp {
                left: left,
                middle: store,
                right: right
            })
        }
    }
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

        ds01: "" =>
             Ok(("", vec![], ""))
        ds02: "#" =>
             Err(())
        ds03: "ab#cde" =>
             Err(())
        ds04: "a" =>
             Ok(("", vec!["a"], ""))
        ds05: "abcde" =>
             Ok(("", vec!["abcde"], ""))
        ds06: r"ab\#cde" =>
             Ok(("", vec!["ab#cde"], ""))
        ds07: r"ab\#c\.de" =>
             Ok(("", vec![r"ab#c\.de"], ""))
        ds08: r"(?P<test>.*[a-zA-z]+)\d+" =>
             Ok(("", vec![r"(?P<test>.*[a-zA-Z]+)\d+"], ""))
        ds09: "#abcde#" =>
             Ok(("", vec!["abcde"], ""))
        ds10: "xy#abcde#z" =>
             Ok(("xy", vec!["abcde"], "z"))
        ds11: "xy##z" =>
             Ok(("xy", vec![""], "z"))
        ds12: r"(?P<test>[a-zA-Z]+);*#(hello\.)\d+#(more.*)" =>
             Ok((r"(?P<test>[a-zA-Z]+);*", vec![r"(hello\.)\d+"], r"(more.*)"))
        ds13: r"xy#ab\#cde#z" =>
             Ok(("xy", vec!["ab#cde"], "z"))
        ds14: "before#a#b#c#d#after" =>
             Ok(("before", vec!["a", "b", "c", "d"], "after"))
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

