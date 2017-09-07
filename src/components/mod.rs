pub mod graphics;
extern crate tcod;
use self::tcod::*;
use self::tcod::console::*;
use self::tcod::input::*;

pub trait Component {
    fn tick(&mut self, delta_time: i32) {
        println!("Tick C");
    }

    fn draw(&mut self, root: &mut Root) {
        println!("Draw C");
    }
}
