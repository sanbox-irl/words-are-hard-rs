#![allow(clippy::bool_comparison)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2018_idioms)]

pub mod gen;

mod challenge_instruction;
mod game;
pub use challenge_instruction::ChallengeInstruction;
pub use game::*;

mod rule;
pub use rule::*;

mod words;
pub use words::{LOWERCASE_CHARS, WORDS};
