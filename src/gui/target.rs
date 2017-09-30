extern crate tcod;
use self::tcod::{RootConsole};
use self::tcod::console::{Offscreen};
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui};
use components::graphics::RenderSystem;

#[derive(Debug, Clone)]
pub struct TargetGui {
    title: String,
}

impl Gui for TargetGui {
    fn new(title: String) -> Self {
        return Self { 
            title: title,
        };
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn get_title(&self) -> &String {
        return &self.title;
    }

    fn process_input(&mut self, key: Key) -> Option<InputStatus> {
        return None;
    }

    fn draw(&mut self, console: &mut RootConsole) {
        let mut screen = Offscreen::new(20, 20);

        RenderSystem::draw_frame(&mut screen, 0, 0, 19, 19, tcod::colors::ORANGE);
        tcod::console::blit(&screen, (0, 0), (20, 20), &mut (*console), (20, 20), 1.0, 1.0);
    }
}
