extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui, GuiKey};
use gui::elements::GuiList;
use components::graphics::RenderSystem;
use components::action::Direction;

#[derive(Debug, Clone)]
pub struct CharacterInfoGui {
    title: String,
}

impl Gui for CharacterInfoGui {
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

    fn process_input(&mut self, key: &Key) -> Option<InputStatus> {
        match Self::check_gui_keys(&key) {
            // Process and move
            Some(guiKey) => {
                match guiKey {
                    GuiKey::Exit => {
                        // Go back to no status
                        return Some(InputStatus::None);
                    }

                    GuiKey::Move(dir) => {
                        match dir {
                            Direction::N => {
                            }
                            Direction::S => {
                            }
                            Direction::E => {
                            }
                            _ => {}
                        }
                    }

                    GuiKey::Confirm => {
                    }
                    _ => {}
                }
            }
            None => {
                // Error message?
            }
        }
        return None;
    }

    fn draw(&self, console: &mut RootConsole, x: i32, y: i32, w: i32, h: i32) {
        let mut gui_screen = Offscreen::new(w, h);

        RenderSystem::draw_frame(&mut *console, x - 1, y - 1, w + 1, h + 1, colors::GREEN);
        tcod::console::blit(
            &gui_screen,
            (0, 0),
            (w, h),
            &mut (*console),
            (x, y),
            1.0,
            1.0,
        );
        console.print_ex(
            x + w - 1,
            y - 1,
            BackgroundFlag::Set,
            TextAlignment::Right,
            self.get_title(),
        );
    }
}

impl CharacterInfoGui {
}
