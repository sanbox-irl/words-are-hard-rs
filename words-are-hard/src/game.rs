use crate::{gen, ChallengeInstruction, Rule, TargetDestination};

/// The main struct of the game.
pub struct Game {
    rules: Vec<Rule>,
    words: Vec<WordData>,
    len: usize,

    current_round: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut rules = vec![Rule::Convert(TargetDestination::default()); 8];
        let mut words = vec![WordData::default(); 8];

        for i in 0..8 {
            // first, generate our new word!
            let (secret, selection_range) = gen::generate_word(&mut rng, &rules[0..i]);

            // and get a new rule!
            rules[i] = gen::generate_rule(i, &mut rng, selection_range);

            // okay GOOD LUCK PLAYER!
            let mut hard_word = secret.to_string();
            for rule in rules[0..=i].iter() {
                hard_word = rule.apply(&hard_word);
            }

            words[i] = WordData {
                secret: secret.to_string(),
                hard_word,
            };
        }

        Game {
            rules,
            words,
            len: 8,
            current_round: 0,
        }
    }

    /// Get a reference to the game's len.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn advance_game(&mut self) {
        self.current_round += 1;
    }

    pub fn round_data(&self) -> Option<RoundData<'_>> {
        if self.len == self.current_round {
            return None;
        }
        let word_data = &self.words[self.current_round];
        let rules = &self.rules[0..=self.current_round];

        Some(RoundData { rules, word_data })
    }
}

impl Game {
    pub fn new_instructions(instructions: &[ChallengeInstruction]) -> Self {
        let len = instructions.len();
        let mut rules = vec![Rule::Convert(TargetDestination::default()); len];
        let mut words = vec![WordData::default(); len];

        for (i, instruction) in instructions.iter().enumerate() {
            rules[i] = instruction.rule;

            let mut hard_word = instruction.word.to_string();
            for rule in rules[0..=i].iter() {
                hard_word = rule.apply(&hard_word);
            }

            let word_data = WordData {
                secret: instruction.word.clone(),
                hard_word,
            };
            words[i] = word_data;
        }

        Self {
            rules,
            words,
            len,
            current_round: 0,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter { game: self, cursor: 0 }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RoundData<'a> {
    pub rules: &'a [Rule],
    pub word_data: &'a WordData,
}

#[derive(Debug, Default, Clone)]
pub struct WordData {
    pub secret: String,
    pub hard_word: String,
}

pub struct Iter<'a> {
    game: &'a Game,
    cursor: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = RoundData<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.game.len() {
            let word_data = &self.game.words[self.cursor];
            let rules = &self.game.rules[0..=self.cursor];
            self.cursor += 1;

            Some(RoundData { rules, word_data })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.game.len - self.cursor, Some(self.game.len - self.cursor))
    }
}
