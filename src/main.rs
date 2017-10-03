mod game;
mod components;
mod world;
mod gui;
mod camera;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
