extern crate tcod;
extern crate specs;
use self::specs::*;

/// TurnState
///     Describes a single turns action
///     Wraps key presses, state the player is in,
///     such as Examining, attacking etc.
#[derive(Debug)]
pub struct TurnStateComponent {
    pub vec: (i32, i32),
    pub action: ActionState,
}

impl Component for TurnStateComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum ActionState {
    NONE,
    MOVE,
    EXAMINE,
    ATTACK,
}

