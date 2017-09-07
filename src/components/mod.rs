pub mod graphics;
extern crate tcod;
use self::tcod::*;
use self::tcod::console::*;
use self::tcod::input::*;
use game::*;

pub trait Component {
    fn tick(&mut self, game: &mut Game, delta_time: i32) {
        println!("Tick C");
    }

    fn draw(&mut self, game: &mut Game) {
        println!("Draw C");
    }
}
