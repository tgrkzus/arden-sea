extern crate tcod;

extern crate specs;
use self::specs::{System, WriteStorage, ReadStorage, Fetch, FetchMut, Join};

use components::position::{CharacterPositionComponent};
use components::state::{TurnStateComponent, ActionState};

pub struct ActionControllerSystem;
impl<'a> System<'a> for ActionControllerSystem {
    type SystemData = (WriteStorage<'a, CharacterPositionComponent>,
                       ReadStorage<'a, TurnStateComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut position, turn) = data;

        for (position, turn) in (&mut position, &turn).join() {
            println!("{:?}", position);
            println!("{:?}", turn);
            match turn.action {
                ActionState::NONE => {
                    println!("NONE action");
                },
                ActionState::MOVE => {
                    println!("MOVE action");
                    position.x += turn.vec.0;
                    position.y += turn.vec.1;
                },
                ActionState::EXAMINE => {
                    println!("EXAMINE action");
                },
                ActionState::ATTACK => {
                    println!("ATTACK action");
                },
                _ => {
                    println!("Default");
                }
            }
        }
    }
}

pub struct ActionGeneratorSystem;
impl<'a> System<'a> for ActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let mut turn = data;

        for turn in (&mut turn).join() {
            turn.action = ActionState::MOVE;
            turn.vec = (1, 1);
        }
    }
}
