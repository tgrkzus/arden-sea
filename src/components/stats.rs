extern crate specs;
use self::specs::*;
use components::*;

/// Defines the generic statistics of a entity
/// 
/// Three types of statistics:
///     Physical:
///         Change dynamically with the state of the entities body.
///         E.g. if you have the body of a robot you're probably pretty strong
///     
///     Mental:
///         General stay with the character and develop as the player progresses
///         to indiciate a level of learning.
///
///     Other:
///         Attached to the 'soul' of a character. Right now is just luck
///
///     Strength: 
///         Physical strength of the entities body
///         this should change with respect to the body
///         of the user. E.g. if you somehow attach a biotic
///         arm that should generally increase your strength.
///
///         This statistic should be use for when entities are performing
///         actions. I.e. opening something, attacking etc.
///
///     Constitution:
///         Defines the resilence of an entities body.
///         Should also change with respect to the entities body.
///         
///         Should be used in response to mostly defensive and encompasses
///         statistics like stamina, endurance etc. 
///
///     Dexterity:
///         Defines the agility of a character. Things like speed, fine motor skills etc.
///         Accuracy and evasion also fall under this. Affected by the state of body,
///         as well as things like weight etc.
///
///     Intelligence:
///         Mental stat. Defines the collective knowledge, intellect, wisdom etc
///         of a character. 
///
///     Charisma:
///         Mental stat. Affects how well this character deals with social situations.
///         Generally increases the likability of a character.
///
///     Willpower:
///         Mental stat. Affects the mental willpower of a character. Their ability to
///         mentally resist pain, mental stress, fear etc.
///
///     Perception:
///         Mental stat. Affects how 'sharp' a character's senses are. Affects ability
///         to deduce information simply from observation. Of course this is useless if
///         a character lacks senses. E.g. a character without ears cannot hear someone
///         walking.
///
///     Luck:
///         Other stat. Voodoo Magical Stuff. TODO maybe remove?
///
#[derive(Debug)]
pub struct StatsComponent{
    pub strength: i32,
    pub constitution: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub charisma: i32,
    pub willpower: i32,
    pub perception: i32,
    pub luck: i32,
}

impl Component for StatsComponent {
    type Storage = VecStorage<Self>;
}

impl StatsComponent {
    pub fn new(strength: i32, constitution: i32, dexterity: i32, 
               intelligence: i32, charisma: i32, willpower: i32, 
               perception: i32, luck: i32) -> Self {
        Self {
            strength: strength,
            constitution: constitution,
            dexterity: dexterity,
            intelligence: intelligence,
            charisma: charisma,
            willpower: willpower,
            perception: perception,
            luck: luck,
        }
    }
}
