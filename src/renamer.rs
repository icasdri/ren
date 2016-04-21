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

fn decompose_source(source_pattern: String, delim: char) -> Result<(String, Vec<String>, String), String> {
    let mut store: Vec<String> = source_pattern.split(delim).map(|e| e.to_owned()).collect();
    
    match store.len() {
        0 | 2 => Err(String::from("hanging delim")),
        1 => Ok((String::new(), store, String::new())),
        _ => {
            let left = store.remove(0);
            let right = store.pop().unwrap();
            Ok((left, store, right))
        }
    }
}

// fn decompose_target(target_pattern: String, delim: char) -> Result<Decomp, DecompErr> {
    // // TODO: actually implement
    // Ok(Decomp {
        // left: String::new(),
        // middle: Vec::new(),
        // right: String::new()
    // })
// }

#[cfg(test)]
mod test_decompose {
    use super::{decompose_source};

    parameterized! {
        test_decompose_source;

        ds01: "" =>
              Ok(("", vec![""], ""))
        ds02: "#" =>
              Err(())
        ds03: "ab#cde" =>
              Err(())
        ds04: "a" =>
              Ok(("", vec!["a"], ""))
        ds05: "abcde" =>
              Ok(("", vec!["abcde"], ""))
        ds06: r"(?P<test>.*[a-zA-Z]+)\d+" =>
              Ok(("", vec![r"(?P<test>.*[a-zA-Z]+)\d+"], ""))
        ds07: "#abcde#" =>
              Ok(("", vec!["abcde"], ""))
        ds08: "xy#abcde#z" =>
              Ok(("xy", vec!["abcde"], "z"))
        ds09: "xy##z" =>
              Ok(("xy", vec![""], "z"))
        ds10: r"(?P<test>[a-zA-Z]+);*#(hello\.)\d+#(more.*)" =>
              Ok((r"(?P<test>[a-zA-Z]+);*", vec![r"(hello\.)\d+"], r"(more.*)"))
        ds11: "before#a#b#c#d#after" =>
              Ok(("before", vec!["a", "b", "c", "d"], "after"))
    }

    fn test_decompose_source(param_source_pattern: &'static str, param_expected: Result<(&'static str, Vec<&'static str>, &'static str), ()>) {
        let delim = '#';
        let source_pattern = param_source_pattern.to_owned();
        let result = super::decompose_source(source_pattern, delim);

        match param_expected {
            Ok(expected) => {
                assert!(result.is_ok());
                let (r1, r2, r3) = result.unwrap();
                let (e1, e2, e3) = expected;
                assert_eq!(r1, e1);
                assert_eq!(r2, e2);
                assert_eq!(r3, e3);
            },
            Err(_) => {
                assert!(result.is_err());
            }
        }
    }
    
    #[test]
    fn ds_different_delim() {
        let result = super::decompose_source("abc%123%xyz".to_owned(), '%');
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("abc".to_owned(), vec!["123".to_owned()], "xyz".to_owned()));
    }
}

