extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use components::position::*;
use game::{WorldAttributes, LogContent};
use world::map::{TileType, Tile, Map};

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
     Fetch<'a, Map>);

    fn run(&mut self, data: Self::SystemData) {
        let (render, position, mut window, log, map) = data;
        // Clear
        window.clear();

        // Make offscreens
        let mut world_screen = Offscreen::new(WORLD_WINDOW_SIZE.0, WORLD_WINDOW_SIZE.1);
        let mut log_screen = Offscreen::new(LOG_SIZE.0, LOG_SIZE.1);

        // Render map
        RenderSystem::draw_tiles(&mut world_screen, &map);
        
        // Render characters
        for (render, position) in (&render, &position).join() {
            world_screen.put_char_ex(position.x, position.y, render.c, colors::RED, colors::BLACK);
        }

        // World window frame + blitting
        RenderSystem::draw_frame(&mut *window, WORLD_OFFSET.0 - 1, WORLD_OFFSET.1 - 1, WORLD_WINDOW_SIZE.0 + 1, WORLD_WINDOW_SIZE.1 + 1, colors::DESATURATED_FLAME);
        tcod::console::blit(&world_screen, (0, 0), WORLD_WINDOW_SIZE,
                      &mut (*window), WORLD_OFFSET, 1.0, 1.0);


        // Write to log
        log_screen.print_rect(0, 0, LOG_SIZE.0, LOG_SIZE.1, log.content.join("\n"));

        // Log window frame + blitting
        RenderSystem::draw_frame(&mut *window, LOG_OFFSET.0 - 1, LOG_OFFSET.1 - 1, LOG_SIZE.0 + 1, LOG_SIZE.1 + 1, colors::DESATURATED_FLAME);
        tcod::console::blit(&log_screen, (0, 0), LOG_SIZE,
                      &mut (*window), LOG_OFFSET, 1.0, 1.0);
        // Flush changes
        window.flush();
    }
}

impl RenderSystem {

    fn draw_tiles(console: &mut Console, map: &Map) {
        for x in 0..WORLD_WINDOW_SIZE.0 {
            for y in 0..WORLD_WINDOW_SIZE.1 {
                match map.get_tile(x as usize, y as usize, 0 as usize) {
                    Some(tile) => { 
                        let mut c: char;
                        let mut foreground: colors::Color;
                        let mut background: colors::Color;
                        match tile.tile_type {
                            TileType::Wall => {
                                c = 'W';
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

    fn draw_frame(console: &mut Console, x: i32, y: i32, width: i32, height: i32, color: colors::Color) {
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
