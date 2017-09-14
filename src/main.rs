mod game;
mod entity;
mod components;


fn main() {
    let mut game = game::Game::new();
    game.run();
}
