extern crate tcod;
use self::tcod::*;
use self::tcod::console::Console;
use self::tcod::console::Offscreen;
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui, GuiKey};
use components::graphics::RenderSystem;
use components::action::Direction;


#[derive(Debug, Clone)]
pub struct GuiList {
    elements: Vec<String>,
    selected: i32,
}

impl GuiList {
    pub fn new() -> Self {
        return Self {
            elements: Vec::new(),
            selected: 0,
        };
    }

    /// Sets the selected object
    pub fn select(&mut self, i: i32) {
        if self.elements.is_empty() {
            panic!("GuiList is empty");
        }

        self.selected = i;
    }

    /// Selects the next object,
    /// if there is no next object it will loop around to the first
    pub fn select_next(&mut self) {
        if self.elements.is_empty() {
            panic!("GuiList is empty");
        }

        self.selected += 1;

        if self.selected as usize >= self.elements.len() {
            self.selected = 0;
        }
    }

    /// Selects the prev object,
    /// if there is no prev object it will loop around to the last
    pub fn select_prev(&mut self) {
        if self.elements.is_empty() {
            panic!("GuiList is empty");
        }

        self.selected -= 1;

        if self.selected < 0 {
            self.selected = (self.elements.len() as i32) - 1;
        }
    }

    /// Returns the selected immutable object
    pub fn get_selected(&self) -> &String {
        if self.elements.is_empty() {
            panic!("GuiList is empty");
        }

        return &self.elements[self.selected as usize];
    }

    /// Returns the selected mutable object
    pub fn get_selected_mut(&mut self) -> &mut String {
        if self.elements.is_empty() {
            panic!("GuiList is empty");
        }

        return &mut self.elements[self.selected as usize];
    }

    /// Get's the currently selected index
    pub fn get_index(&self) -> i32 {
        return self.selected;
    }

    /// Borrows an immutable reference to the internal list
    pub fn get_list(&self) -> &Vec<String> {
        return &self.elements;
    }

    pub fn list_count(&self) -> i32 {
        return self.elements.len() as i32;
    }

    /// Clears the list of str representations
    pub fn clear_list(&mut self) {
        self.elements.clear();
    }

    /// Borrows a mutable reference to the internal list
    pub fn get_list_mut(&mut self) -> &mut Vec<String> {
        return &mut self.elements;
    }

    /// Returns if the list is empty
    pub fn is_list_empty(&self) -> bool {
        return self.elements.is_empty();
    }

    /// Draws this element at the given x, y with the given maximum w and h dimensions
    pub fn draw(&self, console: &mut Offscreen, x: i32, y: i32, w: i32, h: i32) {
        for (i, obj) in self.elements.iter().enumerate() {
            console.print_ex(0, i as i32, BackgroundFlag::Set, TextAlignment::Left, obj);
        }

        for j in 0..w {
            console.set_char_background(j, self.get_index(), colors::WHITE, BackgroundFlag::Set);
            console.set_char_foreground(j, self.get_index(), colors::BLACK);
        }

    }
}
