extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Offscreen;
use components::position::*;
use game::WorldAttributes;

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

        window.horizontal_line(0, 0, 81, BackgroundFlag::None);
        window.horizontal_line(0, 81, 81, BackgroundFlag::None);

        window.vertical_line(0, 0, 81, BackgroundFlag::None);
        window.vertical_line(81, 0, 81, BackgroundFlag::None);

        window.put_char(0, 0, tcod::chars::NW, BackgroundFlag::None);
        window.put_char(0, 81, tcod::chars::SW, BackgroundFlag::None);
        window.put_char(81, 0, tcod::chars::NE, BackgroundFlag::None);
        window.put_char(81, 81, tcod::chars::SE, BackgroundFlag::None);

        tcod::console::blit(&world_screen, (0, 0), (attrs.size.0 as i32, attrs.size.0 as i32),
                      &mut (*window), (1, 1), 1.0, 1.0);
        // Flush changes
        window.flush();
    }
}
