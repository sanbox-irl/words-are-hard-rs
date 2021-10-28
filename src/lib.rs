#![allow(clippy::bool_comparison)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2018_idioms)]

pub mod gen;

mod rule;
pub use rule::*;

mod words;
pub use words::{LOWERCASE_CHARS, WORDS};

pub struct Game<const N: usize> {
    rules: [Rule; N],
    words: [WordData; N],
}

impl Game<8> {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut rules = [Rule::Convert(Convert::default()); 8];
        let mut words: [WordData; 8] = Default::default();

        for i in 0..8 {
            // first, generate our new word!
            let (secret, selection_range) = gen::generate_word(&mut rng, &rules[0..i]);

            // and get a new rule!
            rules[i] = gen::generate_rule(i, &mut rng, selection_range);

            // okay GOOD LUCK PLAYER!
            let mut hard_word = secret.to_string();
            for rule in rules[0..i].iter() {
                hard_word = rule.apply(&hard_word);
            }

            words[i] = WordData { secret, hard_word };
        }

        Game { rules, words }
    }
}

impl<const N: usize> Game<N> {
    pub fn iter(&self) -> Iter<'_, N> {
        Iter {
            game: self,
            cursor: 0,
        }
    }
}

impl Default for Game<8> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RoundData<'a> {
    pub rules: &'a [Rule],
    pub word_data: &'a WordData,
}

#[derive(Debug, Default)]
pub struct WordData {
    pub secret: &'static str,
    pub hard_word: String,
}

pub struct Iter<'a, const N: usize> {
    game: &'a Game<N>,
    cursor: usize,
}

impl<'a, const N: usize> Iterator for Iter<'a, N> {
    type Item = RoundData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < N {
            self.cursor += 1;
            let rules = &self.game.rules[0..self.cursor];
            let word_data = &self.game.words[self.cursor];

            Some(RoundData { rules, word_data })
        } else {
            None
        }
    }
}
