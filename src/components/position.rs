extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::*;
use self::tcod::input::*;

use components::*;

#[derive(Debug)]
pub struct CharacterPositionComponent {
    pub x: i32,
    pub y: i32,
}

impl Component for CharacterPositionComponent {
    type Storage = VecStorage<Self>;
}

