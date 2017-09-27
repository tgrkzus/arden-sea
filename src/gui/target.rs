extern crate tcod;
use self::tcod::{RootConsole};
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui};

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

    }
}
