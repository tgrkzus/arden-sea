extern crate tcod;
use self::tcod::RootConsole;
use self::tcod::console::Root;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage,
                    Fetch, FetchMut, Join, Entities, Entity};

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
                       FetchMut<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, mut camera) = data;


        match camera.get_state() {

            CameraState::Fixed(x, y) => {
                // TODO error check
                camera.set_position((x, y));
            }

            CameraState::Attached(id) => {
                match (&*entities, &positions).join().find(|&(ref e, _)| e.id() == id) {
                    Some((_, position)) => {
                        camera.set_position((position.x, position.y));
                    }

                    None => {
                        panic!("Cannot find entity with id");
                    }
                }
            }
        }
        println!("{:?}", *camera);
    }
}

impl Camera {
    pub fn new() -> Self {
        // Default to attached to player
        return Self {
            x: 0,
            y: 0,
            w: 80,
            h: 80,
            state: CameraState::Attached(0),
        };
    }


    pub fn get_position(&self) -> (i32, i32) {
        return (self.x, self.y);
    }


    pub fn get_size(&self) -> (u32, u32) {
        return (self.w, self.h);
    }

    pub fn set_position(&mut self, p: (i32, i32)) {
        self.x = p.0;
        self.y = p.1;

        if (self.x < self.w as i32 / 2) {
            self.x = self.w as i32 / 2;
        }

        if (self.y < self.h as i32 / 2) {
            self.y = self.h as i32 / 2;
        }
    }

    pub fn get_state(&self) -> CameraState {
        return self.state.clone();
    }
}
