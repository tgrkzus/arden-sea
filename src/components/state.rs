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

/// Various actions an entity can take
///     NONE    - Take no action, consume no AP
///     MOVE_BY - Move by the given vec (i.e. offset)
///     MOVE_TO - Move to the given vec (i.e. set to vec)
///     EXAMINE - Examine the given tile
///     ATTACK  - Attack at the given tile (TODO target WITHIN a tile?)
#[derive(Debug)]
pub enum ActionState {
    NONE,
    MOVE_BY,
    MOVE_TO,
    EXAMINE,
    ATTACK,
}

