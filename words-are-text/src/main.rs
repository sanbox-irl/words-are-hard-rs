use console::style;
use words_are_hard::*;

fn main() {
    let mut guess: String;

    let game = Game::new_instructions([
        ChallengeInstruction::new(Rule::Convert(TargetDestination::new('r', 'e')), "arbitrary"),
        ChallengeInstruction::new(Rule::Convert(TargetDestination::new('i', 't')), "warranties"),
        ChallengeInstruction::new(Rule::Convert(TargetDestination::new('n', 'o')), "signatures"),
        ChallengeInstruction::new(Rule::Remove(Remove('h')), "horoscope"),
        ChallengeInstruction::new(Rule::Duplicate(Duplicate::new('c', 2)), "helicopter"),
        ChallengeInstruction::new(Rule::Switch(TargetDestination::new('c', 'e')), "convicted"),
    ]);

    let console = console::Term::stdout();

    'outer: for round_data in game.iter() {
        let mut first_time = true;
        let mut cheat = false;
        'inner: loop {
            console.clear_screen().unwrap();

            println!("{}: execute from top to bottom", style("Rules").yellow());
            for (i, rule) in round_data.rules.iter().enumerate() {
                println!("{}. {}", style(i + 1).yellow(), rule);
            }

            if cheat {
                cheat = false;
                println!("{}", round_data.word_data.secret);
            }

            println!("Hard Word: {}", style(&round_data.word_data.hard_word).red());
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

            if guess == round_data.word_data.secret {
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
