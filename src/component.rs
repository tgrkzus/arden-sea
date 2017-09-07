use std::fmt::Debug;
use std::marker::Sized;
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
/*
    fn tick(self, delta_time: i32) where Self : Debug + Sized {
        println!("Tick: {:?}", self);
    }
    */


#[derive(Debug, Clone)]
pub struct GraphicsComponent {
    c: char,
    x: i32,
    y: i32,
}

impl Component for GraphicsComponent {
    fn draw(&mut self, root: &mut Root) {
        root.put_char_ex(self.x, self.y, self.c, 
                         colors::RED, colors::BLACK); 
    }
}

impl GraphicsComponent {
    pub fn new(c: char, x: i32, y: i32) -> Self {
        return Self {
            c: c,
            x: x,
            y: y,
        };
    }

    /// Move's a entity to the given coords
    ///
    /// No verification of location
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Offsets the entity
    ///
    /// No verification of location
    pub fn offset(&mut self, x: i32, y: i32) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}
