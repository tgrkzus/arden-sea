extern crate tcod;

/// TurnState
///     Describes a single turns action
///     Wraps key presses, state the player is in,
///     such as Examining, attacking etc.
pub struct TurnState {
    pub key: tcod::input::Key,
    pub action: ActionState,
}

pub enum ActionState {
    NORMAL,
}