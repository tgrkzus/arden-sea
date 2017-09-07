extern crate tcod;
use self::tcod::*;
use self::tcod::console::*;
use self::tcod::input::*;

use components::*;
use game::*;

#[derive(Debug, Clone)]
pub struct PlayerControllerComponent {}

impl Component for PlayerControllerComponent {
    fn tick(&mut self, game: &mut Game, delta_time: i32) {
        println!("Tick C");
    }
}

impl GraphicsComponent {
    pub fn new(c: char, x: i32, y: i32) -> Self {
        return Self {};
    }
}
