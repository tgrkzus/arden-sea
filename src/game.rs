use std::process; 

extern crate tcod;
use self::tcod::RootConsole;
use self::tcod::console::Root;

extern crate specs;
use self::specs::{World, DispatcherBuilder, RunNow};

use components::action::{ActionControllerSystem, ActionGeneratorSystem, ControllerComponent,
                         Controllers, Direction};
use components::player::PlayerActionGeneratorSystem;
use components::graphics::{RenderSystem, CharacterRenderComponent};
use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use components::information::{InformationComponent};
use gui::gui::{Gui};
use gui::target::TargetGui;

use world::map::{Tile, TileType, Map};

/// The input status.
/// This describes what state the player acting is in,
/// and is checked to see if we need to get more player input
/// or should just simulate the game.
///
/// For example if the status is Ok we simulate the game
///             if the status is Fail we recieved invalid input
///             
///             Other statuses indicated we want to take an action but required
///             more information. E.g. we want to examine but need a target now.
///             The concept of this is so we have a chance to visually update the game
///             and perform various checks outside of the PlayerActionGeneratorSystem.
///
///             The turn status should always describe the status of the Player entities
///             ActionState and state information like (Who should we attack) should never be
///             contained within this struct. All actors within the game can take all actions the
///             players take and should all follow the same logic for the best simulation. The
///             TurnStatus only describes the input state!
///
///             The turn status can also describe menu state. Such as a player being in and
///             inventory. Which can then be visual displayed appropriately. Actions such as
///             dropping items or using items should still be encompassed within the ActionState
///             though!
///
///             There's also a Gui status which wraps a Gui object to be drawn on top of the world
///             (after all other rendering is done)
#[derive(Clone)]
pub enum InputStatus <'a> {
    Ok,
    Target,
    Examine,
    Attack,
    Fail,
    Gui { gui: &'a Gui },
}

#[derive(Debug)]
pub struct WorldAttributes {
    pub size: (i32, i32),
}

/// TODO move?
#[derive(Debug)]
pub struct LogContent {
    pub content: Vec<String>,
}

impl LogContent {
    pub fn add_message(&mut self, message: String) {
        self.content.push(message);
    }
}

pub struct Game;

impl Game {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&mut self) { 
        let window: Root = RootConsole::initializer().size(160, 100).title("Game").init();

        let mut world = World::new();

        // Register
        world.register::<CharacterRenderComponent>();
        world.register::<CharacterPositionComponent>();
        world.register::<TurnStateComponent>();
        world.register::<ControllerComponent>();
        world.register::<InformationComponent>();

        // Create entities
        world
            .create_entity()
            .with(ControllerComponent { controller: Controllers::Player })
            .with(CharacterPositionComponent { x: 4, y: 4 })
            .with(CharacterRenderComponent { c: '@' })
            .with(TurnStateComponent {
                direction: Direction::None,
                action: ActionState::None,
            })
            .with(InformationComponent {
                name: "The Player".to_string(),
                description: "This is the player!".to_string(),
            })
            .build();

        world
            .create_entity()
            .with(ControllerComponent { controller: Controllers::Enemy })
            .with(CharacterPositionComponent { x: 7, y: 8 })
            .with(CharacterRenderComponent { c: 'E' })
            .with(TurnStateComponent {
                direction: Direction::None,
                action: ActionState::None,
            })
            .with(InformationComponent {
                name: "An enemy".to_string(),
                description: "This thing wants to kill you!".to_string(),
            })
            .build();

        // Add fetchable resource (Note, this is a move)
        world.add_resource(window);
        world.add_resource(WorldAttributes { size: (80, 80), });
        world.add_resource(LogContent { content: vec!["Welcome to the world!".to_string()] , });
        Self::reset_input_status(&mut world);


        let mut map = Map::new(80, 80, 5);
        /*
        map.set_tile(Tile { tile_type: TileType::Wall }, 4, 5, 0);
        map.set_tile(Tile { tile_type: TileType::Wall }, 5, 4, 0);
        map.set_tile(Tile { tile_type: TileType::Wall }, 6, 4, 0);
        map.set_tile(Tile { tile_type: TileType::Wall }, 4, 8, 0);

        map.set_tile(Tile { tile_type: TileType::Air }, 8, 8, 0);
        */
        world.add_resource(map);

        // Render dispatcher
        let mut renderer = DispatcherBuilder::new()
            .add_thread_local(RenderSystem)
            .build();

        // Simulator dispatcher
        let mut simulator = DispatcherBuilder::new()
            .add(ActionGeneratorSystem, "action_generator", &[])
            .add(
                ActionControllerSystem,
                "action_controller",
                &["action_generator"],
            )
            .add_barrier()
            .build();

        // Game loop
        loop {
            // Draw game
            renderer.dispatch(&mut world.res);

            // Get player turn

            let status = self.get_input(&mut world);
            match status {
                InputStatus::Ok => {
                    // Simulate game
                    simulator.dispatch(&mut world.res);

                    // Game has been simulated reset input state
                    //Game::reset_input_status(&mut world);
                },

                InputStatus::Fail => { 
                    println!("Invalid input");
                },

                // Do nothing if we have another status. The renderer system should then dispatch
                // and this will allow the renderer to read the turn status and do various gui
                // related things (I.e. visually ask more input)
               _ => {}
            }
        }
    }

    pub fn get_input(&mut self, world: &mut World) -> InputStatus {
        // Run the PlayerActionGeneratorSystem
        let mut system = PlayerActionGeneratorSystem;
        system.run_now(&world.res);

        // Borrow, deref and clone the current input status
        return (*(world.read_resource::<InputStatus>())).clone();
    }

    /// Reset input status to default Ok
    fn reset_input_status(world: &mut World) {
        world.add_resource(InputStatus::Ok);
    }
}
