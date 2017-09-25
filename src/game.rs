extern crate tcod;
use self::tcod::{RootConsole};
use self::tcod::console::{Root};

extern crate specs;
use self::specs::{World, DispatcherBuilder};

use components::player_controller::{PlayerControllerSystem};
use components::graphics::{CharacterRenderSystem, CharacterRenderComponent};
use components::position::{CharacterPositionComponent};
use state::{TurnState, ActionState};

pub struct Game;

impl Game {
    pub fn new() -> Self {
        return Self { };
    }

    pub fn run(&mut self) {
        let window: Root = RootConsole::initializer()
            .size(80, 80)
            .title("Game")
            .init();
    
        let mut world = World::new();

        // Register
        world.register::<CharacterRenderComponent>();
        world.register::<CharacterPositionComponent>();

        // Create entities
        world.create_entity()
            .with(CharacterPositionComponent { x: 4, y: 4 })
            .with(CharacterRenderComponent { c: '@' })
            .build();

        // Add fetchable resource (Note, this is a move)
        world.add_resource(window);

        // Render dispatcher 
        let mut renderer = DispatcherBuilder::new()
            .add_thread_local(CharacterRenderSystem)
            .build();

        // Simulator dispatcher
        let mut simulator = DispatcherBuilder::new()
            .add_thread_local(PlayerControllerSystem)
            .build();

        // Game loop (with initial draw)
        loop {
            // Draw game
            renderer.dispatch(&mut world.res);

            // Get input
            let input = self.get_input(&mut world);
            world.add_resource(input);

            // Simulate game
            simulator.dispatch(&mut world.res);
        }
    }

    pub fn get_input(&mut self, world: &mut World) -> TurnState {
        let mut console = world.write_resource::<Root>();
        return TurnState {
            key: (*console).wait_for_keypress(false),
            action: ActionState::NORMAL,
        };
    }
}
