use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Convert(Convert),
    Duplicate(Duplicate),
    Remove(Remove),
}

impl Rule {
    /// Applies the rule to the input, returning the output string.
    pub fn apply(self, input: &str) -> String {
        match self {
            Rule::Convert(cnv_data) => cnv_data.apply(input),
            Rule::Duplicate(dep) => dep.apply(input),
            Rule::Remove(rmv) => rmv.apply(input),
        }
    }

    /// Gets the target of the given rule.
    pub fn target(self) -> char {
        match self {
            Rule::Convert(cnv) => cnv.target,
            Rule::Duplicate(dup) => dup.target,
            Rule::Remove(rmv) => rmv.0,
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Convert(cnv) => write!(f, "{} \u{f061} {}", cnv.target, cnv.replace_with),
            Rule::Duplicate(dep) => write!(f, "{} \u{f057} {}", dep.target, dep.count),
            Rule::Remove(rm) => write!(f, "\u{f1f8} {}", rm.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Convert {
    pub target: char,
    pub replace_with: char,
}

impl Convert {
    pub fn apply(self, input: &str) -> String {
        input.replace(self.target, &format!("{}", self.replace_with))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Duplicate {
    pub target: char,
    pub count: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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
        let ex = Convert {
            target: 'b',
            replace_with: 'a',
        };

        assert_eq!(ex.apply("bob"), "aoa");
        assert_eq!(ex.apply("bbb"), "aaa");
        assert_eq!(ex.apply("aoa"), "aoa");
    }

    #[test]
    fn duplicate() {
        let ex = Duplicate {
            target: 'b',
            count: 3,
        };

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
}
