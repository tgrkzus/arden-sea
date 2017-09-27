mod game;
mod components;
mod world;
mod gui;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
