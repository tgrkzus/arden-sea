extern crate tcod;
use self::tcod::RootConsole;
use self::tcod::console::Root;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage, Fetch, FetchMut, Join,
                  Entities, Entity};

use game::WorldAttributes;
use components::action::{ActionControllerSystem, Direction};
use components::position::CharacterPositionComponent;

/// State of the camera
///     Fixed at an (x, y) world point
///     Attached to an entity id
#[derive(Debug, Clone)]
pub enum CameraState {
    Fixed(i32, i32),
    Attached(u32),
}

/// Camera structure
///     (x, y) center point of the camera
///     (w, h) width and height
///     state, the state the camera is currently in
#[derive(Debug, Clone)]
pub struct Camera {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    state: CameraState,
}

pub struct CameraSystem;
impl<'a> System<'a> for CameraSystem {
    type SystemData = (Entities<'a>,
     ReadStorage<'a, CharacterPositionComponent>,
     FetchMut<'a, Camera>,
     Fetch<'a, WorldAttributes>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, mut camera, attrs) = data;


        match camera.get_state() {

            CameraState::Fixed(x, y) => {
                // TODO error check
                camera.set_position((x, y), (attrs.size.0, attrs.size.1));
            }

            CameraState::Attached(id) => {
                match (&*entities, &positions).join().find(
                    |&(ref e, _)| e.id() == id,
                ) {
                    Some((_, position)) => {
                        camera.set_position((position.x, position.y), (attrs.size.0, attrs.size.1));
                    }

                    None => {
                        panic!("Cannot find entity with id");
                    }
                }
            }
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        // Default to attached to player
        return Self {
            x: 40,
            y: 40,
            w: 80,
            h: 80,
            state: CameraState::Attached(0),
        };
    }

    pub fn get_offset(&self) -> (i32, i32) {
        return (self.x - self.w as i32 / 2, self.y - self.h as i32 / 2);
    }

    pub fn get_position(&self) -> (i32, i32) {
        return (self.x, self.y);
    }


    pub fn get_size(&self) -> (u32, u32) {
        return (self.w, self.h);
    }

    pub fn within_bounds(&self, p: (i32, i32)) -> bool {
        return (p.0 >= self.x - self.w as i32 / 2) && (p.0 < self.x + self.w as i32 / 2) &&
            (p.1 >= self.y - self.h as i32 / 2) &&
            (p.1 < self.y + self.h as i32 / 2);
    }

    pub fn set_position(&mut self, p: (i32, i32), max: (i32, i32)) {
        self.x = p.0;
        self.y = p.1;

        if (self.x < self.w as i32 / 2) {
            self.x = self.w as i32 / 2;
        }

        if self.y < self.h as i32 / 2 {
            self.y = self.h as i32 / 2;
        }

        if self.x > max.0 - self.w as i32 / 2 {
            self.x = max.0 - self.w as i32 / 2;
        }

        if self.y > max.1 - self.h as i32 / 2 {
            self.y = max.1 - self.h as i32 / 2;
        }
    }

    pub fn get_state(&self) -> CameraState {
        return self.state.clone();
    }
}
