extern crate tcod;
use self::tcod::{RootConsole};
use self::tcod::input::{Key};
use game::InputStatus;
use components::action::{Direction};

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
    fn process_input(&mut self, key: Key) -> Option<InputStatus>;

    /// Draw's the Gui element on top of the given console.
    ///     Note: should use an internal offscreen console
    ///     TODO return offscreen with coords to blit onto?
    fn draw(&self, console: &mut RootConsole, x: i32, y: i32, w: i32, h: i32);
}
