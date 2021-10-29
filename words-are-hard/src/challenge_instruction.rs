use std::collections::HashMap;

use crate::Rule;

/// Deserialize the available challenges
pub fn deserialize_challenges() -> HashMap<String, ChallengeInstruction> {
    let challenges = include_str!("../../assets/challenges.json");

    serde_json::from_str(challenges).unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChallengeInstruction {
    pub rule: Rule,
    pub word: &'static str,
}

impl ChallengeInstruction {
    pub const fn new(rule: Rule, word: &'static str) -> Self {
        Self { rule, word }
    }
}
