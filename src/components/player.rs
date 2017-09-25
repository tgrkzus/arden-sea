extern crate tcod;

extern crate specs;
use self::specs::{System, WriteStorage, ReadStorage, Fetch, FetchMut, Join};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};

/*
pub struct PlayerActionGeneratorSystem;
impl<'a> System<'a> for PlayerActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let mut turn = data;

        for turn in (&mut turn).join() {
            println!("player");
            turn.action = ActionState::MOVE_BY;
            turn.vec = (1, 1);
        }
    }
}
*/
