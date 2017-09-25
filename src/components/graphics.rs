extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use components::position::*;
use game::WorldAttributes;

const WORLD_OFFSET: (i32, i32) = (10, 10);

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
     Fetch<'a, WorldAttributes>);

    fn run(&mut self, data: Self::SystemData) {
        let (render, position, mut window, attrs) = data;
        // Clear
        window.clear();

        // Make offscreens
        let mut world_screen = Offscreen::new(attrs.size.0 as i32, attrs.size.1 as i32);

        // Render characters
        for (render, position) in (&render, &position).join() {
            world_screen.put_char_ex(position.x, position.y, render.c, colors::RED, colors::BLACK);
        }

        RenderSystem::draw_frame(&mut *window, WORLD_OFFSET.0 - 1, WORLD_OFFSET.1 - 1, attrs.size.0 + 1, attrs.size.1 + 1, colors::DESATURATED_FLAME);

        tcod::console::blit(&world_screen, (0, 0), (attrs.size.0 as i32, attrs.size.0 as i32),
                      &mut (*window), WORLD_OFFSET, 1.0, 1.0);
        // Flush changes
        window.flush();
    }
}

impl RenderSystem {

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
