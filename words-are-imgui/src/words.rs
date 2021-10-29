use std::collections::BTreeMap;

use dauga::{glam::UVec2, Accumulator, AnyResult, Clock, GuiRenderer, Input, Platform, ThemeManager, Window};
use words_are_hard::ChallengeInstruction;

use crate::gui::Gui;

pub struct Words {
    game_data: BTreeMap<String, Vec<ChallengeInstruction>>,
    gui: Gui,

    imgui_platform: Platform,
    window: Window,
    renderer: GuiRenderer,
    input: Input,
}

impl Words {
    pub fn new() -> AnyResult<Self> {
        let window = Window::new("words are hard", UVec2::new(1920, 1080))?;
        let mut imgui_platform = Platform::new(&window);

        let input = Input::new();
        // this is its whole point!
        let theme_manager = ThemeManager::new(dauga::DummyPreferences);
        theme_manager.initialize_style(imgui_platform.context());

        let renderer = GuiRenderer::new(
            window.gl_context(),
            &mut imgui_platform,
            dauga::smol_rgb::EncodedRgb::new(254, 238, 237, 255),
        )?;

        let game_data = words_are_hard::load_challenges().into_iter().collect();

        let me = Self {
            gui: Gui::new(),
            game_data,
            imgui_platform,
            window,
            renderer,
            input,
        };

        Ok(me)
    }

    pub fn main_loop(&mut self) -> AnyResult {
        // who really owns anyone? maybe we'll change our mind
        let mut event_pump = self.window.event_pump();
        let mut clock = Clock::new();
        let mut accum = Accumulator::new(dauga::FrameRateLimiter::variadic());

        loop {
            clock.start_frame();
            let dt = accum.begin_variadic(&clock);

            // hand our input the event pump from our OS and update it!
            self.input.new_frame();
            if self.input.poll_input(&mut event_pump, &mut self.window) {
                break;
            }

            // the actual Ui stuff.
            {
                let window_data = self.window.window_data();
                let ui = self
                    .imgui_platform
                    .new_frame(&window_data, self.window.mouse_util(), &mut self.input, dt);

                self.gui.draw(ui, &self.game_data);

                // self.layout_manager.draw(
                //     &mut self.tree,
                //     ui,
                //     &self.input,
                //     self.time_machine.capabilities(),
                //     &mut self.tether,
                //     &mut new_window_name,
                // );
                let draw_data = self.imgui_platform.render();

                // Render
                self.renderer.clear_screen(&window_data);
                self.renderer.render_imgui(draw_data);
            }

            // swap the buffer!
            self.window.swap_windows();
        }

        Ok(())
    }
}
