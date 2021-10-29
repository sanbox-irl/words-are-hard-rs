use std::collections::HashMap;
use words_are_hard::{ChallengeInstruction, Game};

use dauga::imgui::Ui;

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
                    ui.text("Rules:");
                    for (i, rule) in round_data.rules.iter().enumerate() {
                        ui.text(format!("{}. {}", i + 1, rule));
                    }

                    ui.spacing();

                    ui.text("Output:");
                    ui.same_line();
                    ui.text_colored(
                        [251.0 / 255.0, 162.0 / 255.0, 204.0 / 255.0, 1.0],
                        &round_data.word_data.hard_word,
                    );

                    ui.spacing();

                    let mut pressed_enter = ui
                        .input_text("What is the Input?", &mut game.guess)
                        .enter_returns_true(true)
                        .build();

                    ui.same_line();
                    pressed_enter |= ui.button(dauga::font_awesome::strs::CHECK);
                    ui.same_line();
                    if ui.button(dauga::font_awesome::strs::TIMES) {
                        game.guess.clear();
                    }

                    if pressed_enter {
                        if round_data.word_data.secret == game.guess {
                            game.game.advance_game();
                        }

                        game.guess.clear();
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
                            },
                        );
                    }
                }
            }
        }
    }
}

pub struct ManagedGame {
    game: Game,
    guess: String,
}
