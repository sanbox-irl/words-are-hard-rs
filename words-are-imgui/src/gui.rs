use std::collections::HashMap;
use words_are_hard::{ChallengeInstruction, Game, RoundData, Rule};

use dauga::{imgui::Ui, smol_rgb::EncodedRgb, utils};

pub struct Gui {
    games: HashMap<String, ManagedGame>,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            games: Default::default(),
        }
    }

    pub fn draw(&mut self, ui: &Ui, game_data: &HashMap<String, Vec<ChallengeInstruction>>) {
        self.menu_bar(ui, game_data);

        let mut game_delete = None;

        for (name, game) in self.games.iter_mut() {
            let mut opened = true;
            if let Some(_t) = ui
                .window(name)
                .always_auto_resize(true)
                .resizable(false)
                .opened(&mut &mut opened)
                .begin()
            {
                if let Some(round_data) = game.game.round_data() {
                    if display_round_data(ui, &round_data, &mut game.guess, &mut game.set_keyboard_focus) {
                        game.game.advance_game();
                    }
                } else {
                    ui.text("Good job! Thanks for playing!");
                }
            }

            if opened == false {
                game_delete = Some(name.clone());
            }
        }

        if let Some(game_del) = game_delete {
            self.games.remove(&game_del);
        }
    }

    pub fn menu_bar(&mut self, ui: &Ui, game_data: &HashMap<String, Vec<ChallengeInstruction>>) {
        if let Some(_t) = ui.begin_main_menu_bar() {
            if let Some(_t) = ui.begin_menu("New Game") {
                for (name, instructions) in game_data.iter().filter(|(n, _)| !n.contains("tutorial")) {
                    if ui
                        .menu_item_config(name)
                        .enabled(!self.games.contains_key(name))
                        .build()
                    {
                        self.games.insert(
                            name.clone(),
                            ManagedGame {
                                game: Game::new_instructions(instructions),
                                guess: String::new(),
                                set_keyboard_focus: true,
                            },
                        );
                    }
                }
            }

            if let Some(_t) = ui.begin_menu("Tutorials") {
                for (name, instructions) in game_data.iter().filter(|(n, _)| n.contains("tutorial")) {
                    if ui
                        .menu_item_config(name)
                        .enabled(!self.games.contains_key(name))
                        .build()
                    {
                        self.games.insert(
                            name.clone(),
                            ManagedGame {
                                game: Game::new_instructions(instructions),
                                guess: String::new(),
                                set_keyboard_focus: true,
                            },
                        );
                    }
                }
            }
        }
    }
}

fn display_round_data(ui: &Ui, round_data: &RoundData, guess: &mut String, set_keyboard_focus: &mut bool) -> bool {
    ui.text("Rules:");
    for (i, rule) in round_data.rules.iter().enumerate() {
        ui.text_colored(
            EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(),
            format!("{}.", i + 1),
        );
        ui.same_line();

        let txt = match rule {
            Rule::Convert(cnv) => format!("{} \u{f061} {}", cnv.target, cnv.destination),
            Rule::Duplicate(dep) => format!("{} \u{f057} {}", dep.target, dep.count),
            Rule::Remove(rm) => format!("\u{f1f8} {}", rm.0),
            Rule::Switch(td) => format!("{} \u{f362} {}", td.target, td.destination),
        };
        ui.text(txt);

        if let Some(_t) = utils::raw_help_anywhere(ui) {
            match rule {
                Rule::Convert(_) => {
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "a \u{f061} b");
                    ui.same_line();
                    ui.text(", converts all 'a' to 'b'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Input: 'adam'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "d \u{f061} r");
                    ui.text("Output: 'aram'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Remember: rules execute top to bottom, and effects can stack:");
                    ui.text("Input: 'eve'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "v \u{f061} e");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "e \u{f061} q");
                    ui.text("Output: 'qqq'");
                }
                Rule::Duplicate(_) => {
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "o \u{f057} 3");
                    ui.same_line();
                    ui.text(", duplicates all 'o' 3 times");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Input: 'snake'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "n \u{f057} 2");
                    ui.text("Output: 'snnake'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Remember: rules execute top to bottom, and effects can stack:");
                    ui.text("Input: 'plasma'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "l \u{f057} 3");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "l \u{f061} a");
                    ui.text("Output: 'paaaasma'");
                }
                Rule::Remove(_) => {
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "\u{f1f8} z");
                    ui.same_line();
                    ui.text(", removes all 'z'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Input: 'horizon'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "\u{f1f8} o");
                    ui.text("Output: 'hrizn'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Remember: rules execute top to bottom, and effects can stack:");
                    ui.text("Input: 'sunny'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "s \u{f061} u");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "\u{f1f8} u");
                    ui.text("Output: 'nny'");
                }
                Rule::Switch(_) => {
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "g \u{f362} c");
                    ui.same_line();
                    ui.text(", each 'g' attempts to switch position with the next 'c'. If no 'c' can be found, the 'g' stays");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Input: 'classification'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "s \u{f362} f");
                    ui.text("Output: 'clafsisication'");

                    ui.spacing();
                    ui.spacing();
                    ui.spacing();

                    ui.text("Remember: rules execute top to bottom, and effects can stack:");
                    ui.text("Input: 'orangutang'");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "o \u{f061} a");
                    ui.text_colored(EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(), "a \u{f362} g");
                    ui.text("Output: 'grgnautana'");
                }
            }
        }
    }

    ui.spacing();

    ui.text("Output:");
    ui.same_line();
    ui.text_colored(
        EncodedRgb::new(251, 162, 204, 255).to_encoded_f32s(),
        &round_data.word_data.hard_word,
    );

    ui.spacing();

    if *set_keyboard_focus {
        ui.set_keyboard_focus_here();
        *set_keyboard_focus = false;
    }
    let mut pressed_enter = ui
        .input_text("What is the Input?", guess)
        .enter_returns_true(true)
        .build();
    if pressed_enter {
        *set_keyboard_focus = true;
    }

    ui.same_line();
    pressed_enter |= ui.button(dauga::font_awesome::strs::CHECK);
    ui.same_line();
    if ui.button(dauga::font_awesome::strs::TIMES) {
        guess.clear();
    }

    if pressed_enter {
        let success = round_data.word_data.secret == *guess;
        guess.clear();

        success
    } else {
        false
    }
}

pub struct ManagedGame {
    game: Game,
    guess: String,
    set_keyboard_focus: bool,
}
