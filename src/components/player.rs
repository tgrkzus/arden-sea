use std::process;

extern crate tcod;
use self::tcod::*;
use self::tcod::console::Root;
use self::tcod::input::KeyCode;

extern crate specs;
use self::specs::{Component, VecStorage, System, WriteStorage, ReadStorage,
                    Fetch, FetchMut, Join};

use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use components::action::{ControllerComponent, Controllers};

use game::InputStatus;

pub struct PlayerActionGeneratorSystem;
impl<'a> System<'a> for PlayerActionGeneratorSystem {
    type SystemData = (WriteStorage<'a, TurnStateComponent>,
                       ReadStorage<'a, ControllerComponent>,
                       FetchMut<'a, RootConsole>,
                       FetchMut<'a, InputStatus>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut turn, controller, mut console, mut status) = data;

        for (turn, controller) in (&mut turn, &controller).join() {
            match controller.controller {
                Controllers::Player => {
                    *status = PlayerActionGeneratorSystem::generate_player_action(turn, &mut console, (*status).clone())
                }, _ => { },
            }
        }
    }
}

impl PlayerActionGeneratorSystem {
    fn generate_player_action(turn: &mut TurnStateComponent, console: &mut Root, status: InputStatus) -> InputStatus {
        let key = (*console).wait_for_keypress(false);

        match status {
            // Examine action
            InputStatus::Examine => {
                match Self::check_directions(key) {
                    // Process and move
                    Some(p) => {
                        turn.action = ActionState::Examine;
                        turn.vec = p;
                        return InputStatus::Ok;
                    }
                    // Do nothing if no value
                    None => { 
                    },
                }
            },

            // Normal action
            _ => {

                // Exit check (TODO disable)
                if key.code == KeyCode::Escape {
                    process::exit(0);
                }

                // Check for directions
                match Self::check_directions(key) {
                    // Process and move
                    Some(p) => {
                        turn.action = ActionState::MoveBy;
                        turn.vec = p;
                        return InputStatus::Ok;
                    }
                    // Do nothing if no value
                    None => { 
                    },
                }

                match Self::check_state_keys(key) {
                    // Set our returned state
                    Some(new_state) => {
                        return new_state;
                    }
                    // Do nothing if no value
                    None => { 
                    },
                }
            }
        }
        // If we get here we've got no valid input
        return InputStatus::Fail;
    }


    fn check_state_keys(key: tcod::input::Key) -> Option<InputStatus> {
        let status: InputStatus;
        if key.code == KeyCode::Char {
            match key.printable {
                // Examine
                'e' => {
                    status = InputStatus::Examine;
                }

                // No key found
                _ => { 
                    return None; 
                },
            }
        }
        else {
            match key.code {
                _ => {
                    return None;
                },
            }
        }

        return Some(status);
    }

    /// Checks the given key input for movement keys.
    ///
    /// Returns (x, y) movement direction (TODO Cardinal enum?)
    ///         None if no valid key
    fn check_directions(key: tcod::input::Key) -> Option<(i32, i32)> {
        let p: (i32, i32);
        if key.code == KeyCode::Char {
            match key.printable {
                'w' => p = (0, -1),
                'a' => p = (-1, 0),
                's' => p = (0, 1),
                'd' => p = (1, 0),
                _ => { return None; },
            }
        }
        else {
            match key.code {
                KeyCode::NumPad1 => {
                    p = (-1, 1);
                },
                KeyCode::NumPad2 => {
                    p = (0, 1);
                },
                KeyCode::NumPad3 => {
                    p = (1, 1);
                },

                KeyCode::NumPad4 => {
                    p = (-1, 0);
                },
                KeyCode::NumPad5 => {
                    // Wait action?
                    p = (0, 0);
                },
                KeyCode::NumPad6 => {
                    p = (1, 0);
                },

                KeyCode::NumPad7 => {
                    p = (-1, -1);
                },
                KeyCode::NumPad8 => {
                    p = (0, -1);
                },
                KeyCode::NumPad9 => {
                    p = (1, -1);
                },
                _ => { return None; },
            }
        }
        return Some(p);
    }
}
