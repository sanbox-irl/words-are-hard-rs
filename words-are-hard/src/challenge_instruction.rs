use std::collections::HashMap;

use crate::Rule;

/// Deserialize the available challenges
pub fn deserialize_challenges<P: AsRef<std::path::Path>>(path: P) -> HashMap<String, Vec<ChallengeInstruction>> {
    let txt = std::fs::read_to_string(path).unwrap();

    serde_json::from_str(&txt).unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChallengeInstruction {
    pub rule: Rule,
    pub word: String,
}

impl ChallengeInstruction {
    pub fn new(rule: Rule, word: &'static str) -> Self {
        Self {
            rule,
            word: word.to_string(),
        }
    }
}
