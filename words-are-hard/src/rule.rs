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
    pub fn convert(target: char, destination: char) -> Rule {
        Rule::Convert(TargetDestination::new(target, destination))
    }

    pub fn duplicate(target: char, count: usize) -> Self {
        Rule::Duplicate(Duplicate::new(target, count))
    }

    pub fn remove(target: char) -> Self {
        Self::Remove(Remove(target))
    }

    pub fn switch(target: char, destination: char) -> Rule {
        Rule::Switch(TargetDestination::new(target, destination))
    }

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
            Rule::Convert(cnv) => write!(f, "Convert {} to {}", cnv.target, cnv.destination),
            Rule::Duplicate(dep) => write!(f, "Duplicate {} {} times", dep.target, dep.count),
            Rule::Remove(rm) => write!(f, "Delete {}", rm.0),
            Rule::Switch(td) => write!(f, "{} switches position with the next {}", td.target, td.destination),
        }
    }
}

fn switcher(td: &TargetDestination, input: &str) -> String {
    let mut output_string = String::new();

    let mut data_vec = vec![];

    for (byte_idx, char) in input.char_indices() {
        if char == td.target {
            // put this at the front...we might not be hte most important...
            data_vec.insert(0, byte_idx);

            // and put this signum...
            output_string.push('Q');
        } else if char == td.destination {
            if let Some(old_byte_idx) = data_vec.pop() {
                // replace 'Q' with another char. Because we are always ascii, this works!
                unsafe {
                    output_string.as_bytes_mut()[old_byte_idx] = td.destination as u8;
                }
                output_string.push(td.target);
            } else {
                output_string.push(td.destination);
            }
        } else {
            output_string.push(char);
        }
    }

    for orphan in data_vec {
        // replace 'Q' with another char. Because we are always ascii, this works!
        unsafe {
            output_string.as_bytes_mut()[orphan] = td.target as u8;
        }
    }

    assert!(output_string.contains('Q') == false);

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

        // make sure we don't miss internals
        assert_eq!(Rule::switch('o', 'l').apply("doorbells"), "dllrbeoos");
    }
}
