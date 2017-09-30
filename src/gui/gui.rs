extern crate tcod;
use self::tcod::{RootConsole};
use self::tcod::input::{Key, KeyCode};
use game::InputStatus;
use components::action::{Direction};
use components::player::PlayerActionGeneratorSystem;

#[derive(Debug)]
pub enum GuiKey {
    Exit,               // Esc
    Move(Direction),    // Directionals
    Confirm,            // Enter
    Info,               // E key (generic 'give me more info' key)
}

pub trait Gui {
    fn new(title: String) -> Self where Self:Sized;

    fn set_title(&mut self, title: String);
    fn get_title(&self) -> &String;

    /// Process the given key input, and update the internal gui state depending on the input.
    ///     Optionally can return a new InputStatus to overwrite the current.
    fn process_input(&mut self, key: &Key) -> Option<InputStatus>;

    /// Draw's the Gui element on top of the given console.
    ///     Note: should use an internal offscreen console
    ///     TODO return offscreen with coords to blit onto?
    fn draw(&self, console: &mut RootConsole, x: i32, y: i32, w: i32, h: i32);

    fn check_gui_keys(key: &tcod::input::Key) -> Option<GuiKey> {
        match PlayerActionGeneratorSystem::check_directions(&key) {
            // Process and move
            Some(p) => {
                return Some(GuiKey::Move(p));
            }
            // Do nothing if no value
            None => { 
                if key.code == KeyCode::Char {
                    match key.printable {
                        'e' => {
                            return Some(GuiKey::Info);
                        }
                        _ => {
                            return None;
                        }
                    }
                }
                else {
                    match key.code {
                        KeyCode::Escape => {
                            return Some(GuiKey::Exit);
                        }
                        KeyCode::Enter | KeyCode::NumPadEnter => {
                            return Some(GuiKey::Confirm);
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            },
        }
    }

}
