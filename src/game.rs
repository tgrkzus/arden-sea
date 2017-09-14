extern crate tcod;
use self::tcod::*;
use self::tcod::input::*;
use std::process;
extern crate specs;
use self::specs::*;

use entity::*;
use components::graphics::*;
use components::position::*;

pub enum InputState {
    NORMAL,
    EXAMINE,
}

pub struct Game {
    pub state: InputState,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            state: InputState::NORMAL,
        };
    }

    pub fn run(&mut self) {
        //e.components.push(Box::new(GraphicsComponent::new('@', 2, 2)));
        //self.entities.push(e);
        
        let mut window : RootConsole = RootConsole::initializer()
            .size(80, 80)
            .title("Game")
            .init();

        let mut world = World::new();
        world.register::<CharacterRenderComponent>();
        world.register::<CharacterPositionComponent>();

        world.create_entity().with(CharacterPositionComponent { x: 4, y: 4 })
            .with(CharacterRenderComponent { c: '@' })
            .build();

        world.add_resource(window);

        let mut dispatcher = DispatcherBuilder::new()
            .add_thread_local(CharacterRenderSystem) 
            .build();

        loop {

            dispatcher.dispatch(&mut world.res);

            /*
               let mut p = (0, 0);

            // player entity
            let mut p_ent = self.player.get_entity();
            if c.code == KeyCode::Char {
            match c.printable {
            // Cardinals
            'A' | 'a' => p = (-1,  0),
            'D' | 'd' => p = ( 1,  0),
            'W' | 'w' => p = ( 0, -1),
            'S' | 's' => p = ( 0,  1),
            'e' => {
            self.state = InputState::EXAMINE;
            continue;
            },
            _ => {
            println!("Invalid input");
            continue;
            },
            }
            }
            else {
            match c.code {
            // Cardinals
            KeyCode::Left   => p = (-1,  0),
            KeyCode::Right  => p = ( 1,  0),
            KeyCode::Up     => p = ( 0, -1),
            KeyCode::Down   => p = ( 0,  1),

            KeyCode::NumPad5 => p = ( 0,  0),

            KeyCode::NumPad4 => p = (-1,  0),
            KeyCode::NumPad6 => p = ( 1,  0),
            KeyCode::NumPad8 => p = ( 0, -1),
            KeyCode::NumPad2 => p = ( 0,  1),

            KeyCode::NumPad7 => p = (-1, -1),
            KeyCode::NumPad9 => p = ( 1, -1),
            KeyCode::NumPad1 => p = (-1,  1),
            KeyCode::NumPad3 => p = ( 1,  1),

            KeyCode::Escape => process::exit(0),
            _ => {
            println!("Invalid input");
            continue;
            },

            }
            }

            match self.state {
            InputState::NORMAL => {
            p_ent.offset(p.0, p.1);
            },
            InputState::EXAMINE => {
            let mut ent_name = String::new();
            let newP = (p_ent.x + p.0, p_ent.y + p.1);
            for e in self.entities.iter() {
            if (e.x, e.y) == newP {
            ent_name = e.ident.clone();
            break;
            }
            }
            println!("Examine {} {}: {}", newP.0, newP.1, ent_name);
            self.state = InputState::NORMAL;
            },
            _ => println!("what"),
            }

            println!("Tick!");

            */

        }
    }
}
