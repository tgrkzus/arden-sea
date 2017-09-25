extern crate tcod;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage, Fetch, FetchMut, Join};

use components::position::{CharacterPositionComponent};
use components::state::{TurnStateComponent, ActionState};

#[derive(Debug)]
pub struct ControllerComponent {
    pub controller: Controllers,
}

impl Component for ControllerComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum Controllers {
    PASSIVE,
    PLAYER,
    ENEMY,
}


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
                ActionState::MOVE_BY => {
                    println!("MOVE action");
                    position.x += turn.vec.0;
                    position.y += turn.vec.1;
                },
                ActionState::MOVE_TO => {
                    println!("MOVE action");
                    position.x = turn.vec.0;
                    position.y = turn.vec.1;
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
    type SystemData = (WriteStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, ControllerComponent>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut turn, controller) = data;

        for (turn, controller) in (&mut turn, &controller).join() {
            match controller.controller {
                Controllers::PASSIVE => {
                    println!("PASSIVE controller");
                },
                Controllers::PLAYER => {
                    println!("PLAYER controller");
                    turn.action = ActionState::MOVE_BY;
                    turn.vec = (1, 0);
                },
                Controllers::ENEMY => {
                    println!("ENEMY controller");
                    turn.action = ActionState::MOVE_BY;
                    turn.vec = (1, 1);
                },
                _ => {
                    println!("Default");
                },
            }
        }
    }
}
