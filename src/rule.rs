use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rule {
    Convert(TargetDestination),
    Duplicate(Duplicate),
    Remove(Remove),
    Switch(TargetDestination),
}

impl Rule {
    /// Applies the rule to the input, returning the output string.
    pub fn apply(self, input: &str) -> String {
        match self {
            Rule::Convert(cnv_data) => input.replace(cnv_data.target, &format!("{}", cnv_data.destination)),
            Rule::Duplicate(dep) => dep.apply(input),
            Rule::Remove(rmv) => rmv.apply(input),
            Rule::Switch(target_destination) => switcher(&target_destination, input),
        }
    }

    /// Gets the target of the given rule.
    pub fn target(self) -> char {
        match self {
            Rule::Convert(cnv) => cnv.target,
            Rule::Duplicate(dup) => dup.target,
            Rule::Remove(rmv) => rmv.0,
            Rule::Switch(td) => td.target,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Convert(cnv) => write!(f, "{} \u{f061} {}", cnv.target, cnv.destination),
            Rule::Duplicate(dep) => write!(f, "{} \u{f057} {}", dep.target, dep.count),
            Rule::Remove(rm) => write!(f, "\u{f1f8} {}", rm.0),
            Rule::Switch(td) => write!(f, "{} \u{f07e} {}", td.target, td.destination),
        }
    }
}

fn switcher(td: &TargetDestination, input: &str) -> String {
    let mut output_string = String::new();

    let mut char_iter = input.chars();

    while let Some(char) = char_iter.next() {
        if char == td.target {
            // look forward now...
            let mut mini_output = String::new();
            mini_output.push(td.target);

            for char in &mut char_iter {
                if char == td.destination {
                    // lol i REFUSE to do this slower
                    unsafe {
                        mini_output.as_bytes_mut()[0] = td.destination as u8;
                    }
                    mini_output.push(td.target);
                    break;
                } else {
                    mini_output.push(char);
                }
            }
            output_string.push_str(&mini_output);
        } else {
            output_string.push(char);
        }
    }

    output_string
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct TargetDestination {
    pub target: char,
    pub destination: char,
}

impl TargetDestination {
    pub fn new(target: char, destination: char) -> Self {
        Self { target, destination }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Duplicate {
    pub target: char,
    pub count: usize,
}

impl Duplicate {
    pub fn new(target: char, count: usize) -> Self {
        Self { target, count }
    }
}

impl Duplicate {
    pub fn apply(self, input: &str) -> String {
        input.chars().fold(String::new(), |mut output, chr| {
            let amount = if chr == self.target { self.count } else { 1 };

            for _ in 0..amount {
                output.push(chr);
            }

            output
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Remove(pub char);

impl Remove {
    pub fn apply(self, input: &str) -> String {
        input.chars().fold(String::new(), |mut output, chr| {
            if chr == self.0 {
                // do nothing, since this will effectively remove
                // the target char
            } else {
                output.push(chr);
            }

            output
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let ex = Rule::Convert(TargetDestination {
            target: 'b',
            destination: 'a',
        });

        assert_eq!(ex.apply("bob"), "aoa");
        assert_eq!(ex.apply("bbb"), "aaa");
        assert_eq!(ex.apply("aoa"), "aoa");
    }

    #[test]
    fn duplicate() {
        let ex = Duplicate { target: 'b', count: 3 };

        assert_eq!(ex.apply("bob"), "bbbobbb");
        assert_eq!(ex.apply("bbb"), "bbbbbbbbb");
        assert_eq!(ex.apply("aoa"), "aoa");
    }

    #[test]
    fn delete() {
        let ex = Remove('b');

        assert_eq!(ex.apply("bob"), "o");
        assert_eq!(ex.apply("bbb"), "");
        assert_eq!(ex.apply("aoa"), "aoa");
    }

    #[test]
    fn switch() {
        let ex = TargetDestination {
            target: 'a',
            destination: 'b',
        };

        assert_eq!(switcher(&ex, "abba"), "baba");
        assert_eq!(switcher(&ex, "aobobabrt"), "boaobbart");
        assert_eq!(switcher(&ex, "bca"), "bca");
    }
}
