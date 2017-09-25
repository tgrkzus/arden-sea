use std::process;

extern crate tcod;
use self::tcod::RootConsole;
use self::tcod::console::Root;

extern crate specs;
use self::specs::{World, DispatcherBuilder, RunNow};

use components::action::{ActionControllerSystem, ActionGeneratorSystem, ControllerComponent,
                         Controllers};
use components::player::PlayerActionGeneratorSystem;
use components::graphics::{CharacterRenderSystem, CharacterRenderComponent};
use components::position::CharacterPositionComponent;
use components::state::{TurnStateComponent, ActionState};

#[derive(Clone)]
pub enum TurnStatus {
    OK,
    FAIL,
}

pub struct Game;

impl Game {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn run(&mut self) {
        let window: Root = RootConsole::initializer().size(80, 80).title("Game").init();

        let mut world = World::new();

        // Register
        world.register::<CharacterRenderComponent>();
        world.register::<CharacterPositionComponent>();
        world.register::<TurnStateComponent>();
        world.register::<ControllerComponent>();

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

        // Add fetchable resource (Note, this is a move)
        world.add_resource(window);
        world.add_resource(TurnStatus::FAIL);

        // Render dispatcher
        let mut renderer = DispatcherBuilder::new()
            .add_thread_local(CharacterRenderSystem)
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
