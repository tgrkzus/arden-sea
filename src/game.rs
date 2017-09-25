use std::process; 
extern crate tcod;
use self::tcod::RootConsole;
use self::tcod::console::Root;

extern crate specs;
use self::specs::{World, DispatcherBuilder, RunNow};

use components::action::{ActionControllerSystem, ActionGeneratorSystem, ControllerComponent,
                         Controllers};
use components::player::PlayerActionGeneratorSystem;
use components::graphics::{RenderSystem, CharacterRenderComponent};
use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};
use components::tile::{TileComponent, TileType};

#[derive(Clone)]
pub enum TurnStatus {
    OK,
    FAIL,
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
        world.register::<TileComponent>();

        // Create entities
        world
            .create_entity()
            .with(ControllerComponent { controller: Controllers::Player })
            .with(CharacterPositionComponent { x: 4, y: 4 })
            .with(CharacterRenderComponent { c: '@' })
            .with(TurnStateComponent {
                vec: (0, 0),
                action: ActionState::None,
            })
            .build();

        world
            .create_entity()
            .with(ControllerComponent { controller: Controllers::Enemy })
            .with(CharacterPositionComponent { x: 7, y: 8 })
            .with(CharacterRenderComponent { c: 'E' })
            .with(TurnStateComponent {
                vec: (0, 0),
                action: ActionState::None,
            })
            .build();

        // Tile/walls
        world
            .create_entity()
            .with(CharacterPositionComponent { x: 4, y: 3 })
            .with(CharacterRenderComponent { c: 'W' })
            .with(TileComponent { tile_type: TileType::Impassable })
            .build();

        world
            .create_entity()
            .with(CharacterPositionComponent { x: 4, y: 7 })
            .with(CharacterRenderComponent { c: 'F' })
            .with(TileComponent { tile_type: TileType::Passable })
            .build();

        // Add fetchable resource (Note, this is a move)
        world.add_resource(window);
        world.add_resource(TurnStatus::FAIL);
        world.add_resource(WorldAttributes { size: (80, 80), });
        world.add_resource(LogContent { content: vec!["Welcome to the world!".to_string()] , });

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
                TurnStatus::OK => {
                    // Simulate game
                    simulator.dispatch(&mut world.res);
                },
                _ => { println!("Invalid input"); },
            }
        }
    }

    pub fn get_input(&mut self, world: &mut World) -> TurnStatus {
        // Run the PlayerActionGeneratorSystem
        let mut system = PlayerActionGeneratorSystem;
        system.run_now(&world.res);

        // Borrow, deref and clone the current turn status
        return (*(world.read_resource::<TurnStatus>())).clone();
    }
}
