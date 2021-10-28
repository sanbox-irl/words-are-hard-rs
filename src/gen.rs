use rand::prelude::*;

use crate::*;

/// Generates a new backing word
pub fn generate_word(
    rng: &mut ThreadRng,
    previous_rules: &[Rule],
) -> (&'static str, impl Iterator<Item = char> + Clone) {
    let round = previous_rules.len();

    // these are our choices
    let choices: std::collections::HashSet<_> = previous_rules.iter().map(|v| v.target()).collect();

    let word = if round < 5 {
        *WORDS.choose(rng).unwrap()
    } else {
        // make it a pipelined selection...
        WORDS
            .iter()
            .filter(|w| w.chars().any(|chr| choices.contains(&chr)))
            .choose(rng)
            .unwrap()
    };

    let selection: Vec<_> = if round < 2 {
        let mut chars = word.chars();
        vec![chars.next().unwrap(), chars.last().unwrap()]
    } else {
        word.chars()
            .filter(move |v| choices.contains(v) == false)
            .collect()
    };

    (word, selection.into_iter())
}

/// Generates a new rule
pub fn generate_rule(
    round: usize,
    rng: &mut ThreadRng,
    choices: impl Iterator<Item = char> + Clone,
) -> Rule {
    // okay if we're in the first three rounds, we ALWAYS do a conversion rule...
    if round < 3 {
        let target = choices.choose(rng).unwrap();
        let replace_with = rng.gen_range(LOWERCASE_CHARS);

        let cnv = Rule::Convert(TargetDestination {
            target,
            destination: replace_with,
        });

        return cnv;
    }

    // on round 4, we *always* generate a weird rule...
    if round == 4 {
        let target = choices.clone().choose(rng).unwrap();

        return match rng.gen_range(0..=2) {
            0 => {
                let count = rng.gen_range(2..5);

                Rule::Duplicate(Duplicate { target, count })
            }
            1 => Rule::Remove(Remove(target)),
            2 => Rule::Switch(TargetDestination {
                target,
                destination: choices.choose(rng).unwrap(),
            }),
            _ => unimplemented!(),
        };
    }

    // on subsequent rounds, we pick a random rule...
    let number: usize = rng.gen_range(0..10);
    match number {
        0..=6 => {
            let target = choices.choose(rng).unwrap();
            let replace_with = rng.gen_range(LOWERCASE_CHARS);

            Rule::Convert(TargetDestination {
                target,
                destination: replace_with,
            })
        }
        7..=8 => {
            let target = choices.choose(rng).unwrap();
            let count = rng.gen_range(2..5);

            Rule::Duplicate(Duplicate { target, count })
        }
        9 => {
            let target = choices.choose(rng).unwrap();
            Rule::Remove(Remove(target))
        }
        _ => unimplemented!(),
    }
}
