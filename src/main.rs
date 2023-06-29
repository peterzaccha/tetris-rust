mod game;

use game::Game;
use macroquad::{
    color::{BLUE, DARKGRAY, GREEN, RED, YELLOW},
    input::KeyCode,
    prelude::{is_key_down, Vec2},
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    ui::root_ui,
    window::{clear_background, next_frame, screen_height, screen_width},
};

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.render(screen_width(), screen_height());

        if is_key_down(KeyCode::Left) {
            game.keydown(KeyCode::Left);
        } else if is_key_down(KeyCode::Right) {
            game.keydown(KeyCode::Right);
        } else if is_key_down(KeyCode::Down) {
            game.keydown(KeyCode::Down);
        } else if is_key_down(KeyCode::Space) {
            game.keydown(KeyCode::Space);
        }

        game.update();
        next_frame().await
    }
}
