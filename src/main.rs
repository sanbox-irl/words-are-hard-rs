mod rule;
use rule::*;

mod words;
pub use words::{LOWERCASE_CHARS, WORDS};

use console::style;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut rules = vec![];
    let input = std::io::stdin();
    let mut guess = String::new();

    let mut console = console::Term::stdout();

    'outer: for _ in 0..7 {
        // first, generate our new word!
        let backing_word = *WORDS.choose(&mut rng).unwrap();

        // then make a new rule...
        let number: usize = rng.gen_range(0..10);
        let new_rule = match number {
            0..=7 => {
                let target = backing_word.chars().choose(&mut rng).unwrap();
                let replace_with = rng.gen_range(LOWERCASE_CHARS);

                Rule::Convert(Convert {
                    target,
                    replace_with,
                })
            }
            8 => {
                let target = backing_word.chars().choose(&mut rng).unwrap();
                let count = rng.gen_range(2..5);

                Rule::Duplicate(Duplicate { target, count })
            }
            9 => {
                let target = backing_word.chars().choose(&mut rng).unwrap();
                Rule::Remove(Remove(target))
            }
            _ => unimplemented!(),
        };

        rules.push(new_rule);

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
