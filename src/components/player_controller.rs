extern crate specs;
extern crate tcod;
use self::specs::{System, WriteStorage, Fetch, Join};
use components::position::{CharacterPositionComponent};
use state::{TurnState, ActionState};

pub struct PlayerControllerSystem;
impl<'a> System<'a> for PlayerControllerSystem {
    type SystemData = (WriteStorage<'a, CharacterPositionComponent>,
                       Fetch<'a, TurnState>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut position, turn) = data;

        for position in (&mut position).join() {
            let c: tcod::input::Key = turn.key;
            let mut p: (i32, i32) = (0, 0);
            if c.code == tcod::input::KeyCode::Char {
                match c.printable {
                    // Cardinals
                    'A' | 'a' => p = (-1,  0),
                    'D' | 'd' => p = ( 1,  0),
                    'W' | 'w' => p = ( 0, -1),
                    'S' | 's' => p = ( 0,  1),
                    _ => {
                        println!("Invalid input");
                    }
                }
            }

            position.x += p.0;
            position.y += p.1;

        }
    }
}
