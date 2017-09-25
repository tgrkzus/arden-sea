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
///     None    - Take no action, consume no AP
///     MoveBy  - Move by the given vec (i.e. offset)
///     MoveTo  - Move to the given vec (i.e. set to vec)
///     Examine - Examine the given tile
///     Attack  - Attack at the given tile (TODO target WITHIN a tile?)
#[derive(Debug)]
pub enum ActionState {
    None,
    MoveBy,
    MoveTo,
    Examine,
    Attack,
}
