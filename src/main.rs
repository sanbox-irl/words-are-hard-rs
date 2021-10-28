#![allow(clippy::bool_comparison)]
#![deny(rust_2021_compatibility)]
#![deny(rust_2018_idioms)]

mod gen;
mod rule;
use rule::*;

mod words;
pub use words::{LOWERCASE_CHARS, WORDS};

use console::style;

fn main() {
    let mut rng = rand::thread_rng();
    let mut rules = vec![];
    let mut guess: String;

    let console = console::Term::stdout();

    'outer: for _ in 0..8 {
        // first, generate our new word!
        let (backing_word, selection_range) = gen::generate_word(&mut rng, &rules);

        // and get a new rule!
        gen::generate_rule(&mut rules, &mut rng, selection_range);

        // okay GOOD LUCK PLAYER!
        let mut first_time = true;
        let mut cheat = false;
        'inner: loop {
            console.clear_screen().unwrap();

            let mut hard_word = backing_word.to_string();
            println!("{}: execute from top to bottom", style("Rules").yellow());
            for (i, rule) in rules.iter().enumerate() {
                println!("{}. {}", style(i + 1).yellow(), rule);
                // apply the rule!
                hard_word = rule.apply(&hard_word);
            }

            if cheat {
                cheat = false;
                println!("{}", backing_word);
            }

            println!("Hard Word: {}", style(hard_word).red());
            println!();
            print!("What was the {}?", style("original word").yellow());

            if first_time {
                println!();
            } else {
                println!(" ('exit' to quit)");
            }
            println!();

            // wait for enter...
            guess = dialoguer::Input::new().interact_text_on(&console).unwrap();
            guess = guess.trim().to_lowercase();

            if guess == backing_word {
                guess.clear();
                break 'inner;
            }

            if guess == "exit" {
                break 'outer;
            }

            if guess == "cheat" {
                cheat = true;
            }

            first_time = false;
        }
    }

    println!("Good job on winning, or giving up!");
}
