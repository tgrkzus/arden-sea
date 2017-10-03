extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use components::position::*;
use game::{WorldAttributes, LogContent, InputStatus, GuiType};
use gui::gui::Gui;
use gui::target::TargetGui;
use world::map::{TileType, Tile, Map};
use camera::{Camera, CameraState};

const WORLD_OFFSET: (i32, i32) = (10, 10);
const WORLD_WINDOW_SIZE: (i32, i32) = (80, 80);

const LOG_OFFSET: (i32, i32) = (92, 10);
const LOG_SIZE: (i32, i32) = (60, 80);

#[derive(Debug)]
pub struct CharacterRenderComponent {
    pub c: char,
}

impl Component for CharacterRenderComponent {
    type Storage = VecStorage<Self>;
}


pub struct RenderSystem;
impl<'a> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, CharacterRenderComponent>,
     ReadStorage<'a, CharacterPositionComponent>,
     FetchMut<'a, RootConsole>,
     Fetch<'a, LogContent>,
     Fetch<'a, Map>,
     Fetch<'a, InputStatus>,
     Fetch<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (render, position, mut window, log, map, input_status, camera) = data;

        window.set_default_foreground(colors::WHITE);
        window.set_default_background(colors::BLACK);

        // Clear
        window.clear();

        // Make offscreens
        let mut world_screen = Offscreen::new(WORLD_WINDOW_SIZE.0, WORLD_WINDOW_SIZE.1);
        let mut log_screen = Offscreen::new(LOG_SIZE.0, LOG_SIZE.1);

        // Render map
        RenderSystem::draw_tiles(&mut world_screen, &map, &camera);
        
        let c_offset = camera.get_offset();
        // Render characters
        for (render, position) in (&render, &position).join().filter(|&(_, ref p)| camera.within_bounds((p.x, p.y))) {
            world_screen.put_char_ex(position.x - c_offset.0, position.y - c_offset.1, render.c, colors::RED, colors::BLACK);
        }

        // World window frame + blitting
        RenderSystem::draw_frame(&mut *window, WORLD_OFFSET.0 - 1, WORLD_OFFSET.1 - 1, WORLD_WINDOW_SIZE.0 + 1, WORLD_WINDOW_SIZE.1 + 1, colors::DESATURATED_BLUE);
        tcod::console::blit(&world_screen, (0, 0), WORLD_WINDOW_SIZE,
                      &mut (*window), WORLD_OFFSET, 1.0, 1.0);


        // Write to log
        log_screen.print_rect(0, 0, LOG_SIZE.0, LOG_SIZE.1, log.content.join("\n"));

        // Log window frame + blitting
        RenderSystem::draw_frame(&mut *window, LOG_OFFSET.0 - 1, LOG_OFFSET.1 - 1, LOG_SIZE.0 + 1, LOG_SIZE.1 + 1, colors::DESATURATED_BLUE);
        tcod::console::blit(&log_screen, (0, 0), LOG_SIZE,
                      &mut (*window), LOG_OFFSET, 1.0, 1.0);

        // Draw input status bar
        window.set_default_foreground(colors::WHITE);
        let status: String;
        match *input_status {
            InputStatus::None => {
                status = "".to_string();
            },
            InputStatus::Ok => {
                status = "Ok".to_string();
            },
            InputStatus::Target => {
                status = "Target".to_string();
            },
            InputStatus::Examine => {
                status = "Examine".to_string();
            },
            InputStatus::Attack => {
                status = "Attack".to_string();
            },
            InputStatus::Fail => {
                status = "Invalid Input".to_string();
                window.set_default_background(colors::RED);
            },
            InputStatus::Gui(ref action, ref gui) => {
                status = "Gui".to_string();
                Self::draw_gui(&mut *window, &gui);
            },
            _ => {
                panic!("Unknown gui type");
            },
        }
        window.print_ex(WORLD_OFFSET.0, WORLD_OFFSET.1 + WORLD_WINDOW_SIZE.1 + 1, BackgroundFlag::Set, TextAlignment::Left, status);
        
        // Flush changes
        window.flush();
    }
}

impl RenderSystem {
    fn draw_gui(mut console: &mut RootConsole, gui: &GuiType) {
        //let mut gui_screen = Offscreen::new(LOG_SIZE.0, LOG_SIZE.1); 
        //RenderSystem::draw_frame(&mut *window, WORLD_OFFSET.0 - 1, WORLD_OFFSET.1 - 1, WORLD_WINDOW_SIZE.0 + 1, WORLD_WINDOW_SIZE.1 + 1, colors::DESATURATED_BLUE); 
        //tcod::console::blit(&gui_screen, (0, 0), WORLD_WINDOW_SIZE,
        //             &mut (*window), WORLD_OFFSET, 1.0, 1.0);

        // Fade background
        let w = console.width();
        let h = console.height();

        for x in 0..w {
            for y in 0..h {
                //console.set_char_background(x, y, colors::GREY, BackgroundFlag::);
                console.set_char_foreground(x, y, colors::GREY);
            }
        }

        match *gui {
            GuiType::Target(ref targetGui) => {
                let w = 30;
                let h = 30;
                let x = WORLD_OFFSET.0 + WORLD_WINDOW_SIZE.0 / 2 - w / 2;
                let y = WORLD_OFFSET.1 + WORLD_WINDOW_SIZE.1 / 2 - h / 2;
                targetGui.draw(&mut console, x, y, w, h);
            }

            _ => {
                panic!("Unknown gui type");
            }
        }
    }

    fn draw_tiles(console: &mut Console, map: &Map, camera: &Camera) {
        let c_offset = camera.get_offset();
        for x in 0..WORLD_WINDOW_SIZE.0 {
            for y in 0..WORLD_WINDOW_SIZE.1 {
                match map.get_tile(x as usize + c_offset.0 as usize, y as usize + c_offset.1 as usize, 0 as usize) {
                    Some(tile) => { 
                        let mut c: char;
                        let mut foreground: colors::Color;
                        let mut background: colors::Color;
                        match tile.tile_type {
                            TileType::Wall => {
                                c = tcod::chars::BLOCK2;
                                foreground = colors::GREY;
                                background = colors::BLACK;
                            }
                            TileType::Ground => {
                                c = '.';
                                foreground = colors::DARKEST_GREY;
                                background = colors::BLACK;
                            }
                            TileType::Air => {
                                c = ' ';
                                foreground = colors::LIGHT_GREY;
                                background = colors::DARK_GREY;
                            }
                        }
                        console.put_char_ex(x, y, c, foreground, background);
                    },
                    None => panic!("No tile what"),
                }
            }
        }

    }

    pub fn draw_frame(console: &mut Console, x: i32, y: i32, width: i32, height: i32, color: colors::Color) {
        console.set_default_foreground(color);
        console.horizontal_line(x, y, width, BackgroundFlag::None);
        console.horizontal_line(x, y + height, width, BackgroundFlag::None);

        console.vertical_line(x, y, height, BackgroundFlag::None);
        console.vertical_line(x + width, y, height, BackgroundFlag::None);

        console.put_char(x, y, tcod::chars::NW, BackgroundFlag::None);
        console.put_char(x, y + height, tcod::chars::SW, BackgroundFlag::None);
        console.put_char(x + width, y, tcod::chars::NE, BackgroundFlag::None);
        console.put_char(x + width, y + height, tcod::chars::SE, BackgroundFlag::None);
    }
}
