extern crate specs;
use self::specs::*;

extern crate tcod;
use self::tcod::*;
use components::position::*;

#[derive(Debug)]
pub struct CharacterRenderComponent {
    pub c: char,
}

impl Component for CharacterRenderComponent {
    type Storage = VecStorage<Self>;
}


pub struct CharacterRenderSystem;
impl<'a> System<'a> for CharacterRenderSystem {
    type SystemData = (ReadStorage<'a, CharacterRenderComponent>,
                       ReadStorage<'a, CharacterPositionComponent>,
                       FetchMut<'a, RootConsole>);

    fn run(&mut self, data: Self::SystemData) {
        let (render, position, mut window) = data;
        // Clear
        window.clear();


        // process input

        for (render, position) in (&render, &position).join() {
            window.put_char_ex(position.x, position.y, render.c,
                               colors::RED, colors::BLACK);
        }
        // Flush changes
        window.flush();
    }
}
