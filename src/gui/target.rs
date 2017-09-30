extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui};
use gui::elements::{GuiList};
use components::graphics::RenderSystem;

#[derive(Debug, Clone)]
pub struct TargetGui {
    title: String,
    list: GuiList,
}

impl Gui for TargetGui {
    fn new(title: String) -> Self {
        return Self { 
            title: title,
            list: GuiList::new(),
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

    fn draw(&self, console: &mut RootConsole, x: i32, y: i32, w: i32, h: i32) {
        let mut gui_screen = Offscreen::new(30, 30); 

        gui_screen.print_ex(0, 0, BackgroundFlag::Set, TextAlignment::Left, "Some stuff!".to_string());

        RenderSystem::draw_frame(&mut *console, x - 1, y - 1, w + 1, h + 1, colors::GREEN); 
        tcod::console::blit(&gui_screen, (0, 0), (w, h),
        &mut (*console), (x, y), 1.0, 1.0);
        console.print_ex(x + w - 1, y - 1, BackgroundFlag::Set, TextAlignment::Right, self.get_title());
    }
}

impl TargetGui {
    pub fn get_list_mut(&mut self) -> &mut Vec<String> {
        return &mut self.list.get_list_mut();
    }
}
