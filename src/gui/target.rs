extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use self::tcod::input::{Key, KeyCode};

use game::InputStatus;
use gui::gui::{Gui, GuiKey};
use gui::elements::{GuiList};
use components::graphics::RenderSystem;
use components::action::Direction;

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
                                self.list.select_prev();
                            }
                            Direction::S => {
                                self.list.select_next();
                            }
                            Direction::E => {
                                println!("Selected: {}:{}", self.list.get_index(), self.list.get_selected());
                            }
                            _ => {

                            }
                        }
                    }

                    GuiKey::Confirm => {
                        println!("Selected: {}:{}", self.list.get_index(), self.list.get_selected());
                    }

                    _ => {
                        println!("{:?}", guiKey);
                    }
                }
            }
            None => {                         
                // Error message?
            },
        }
        return None;
    }

    fn draw(&self, console: &mut RootConsole, x: i32, y: i32, w: i32, h: i32) {
        let mut gui_screen = Offscreen::new(30, 30); 

        self.list.draw(&mut gui_screen, 0, 0, w, h);
        RenderSystem::draw_frame(&mut *console, x - 1, y - 1, w + 1, h + 1, colors::GREEN); 
        tcod::console::blit(&gui_screen, (0, 0), (w, h),
        &mut (*console), (x, y), 1.0, 1.0);
        console.print_ex(x + w - 1, y - 1, BackgroundFlag::Set, TextAlignment::Right, self.get_title());
    }
}

impl TargetGui {
    pub fn add_to_list(&mut self, representation: String) {
        self.list.get_list_mut().push(representation);
    }

    pub fn clear_list(&mut self) {
        self.list.clear_list();
    }

    pub fn is_list_empty(&self) -> bool {
        return self.list.is_list_empty();
    }

    pub fn list_count(&self) -> i32 {
        return self.list.list_count();
    }
}
