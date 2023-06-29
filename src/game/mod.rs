use macroquad::{
    color::WHITE,
    input::KeyCode,
    prelude::{BLACK, GREEN},
    text::draw_text,
    time::{get_fps, get_time},
    window::clear_background,
};

use self::board::Board;

pub mod board;
pub mod tetro;

pub struct Game {
    board: Board,
    last_update: f64,
    last_input: f64,
    speed: f64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            last_update: get_time(),
            last_input: get_time(),
            speed: 0.5,
        }
    }

    pub fn keydown(&mut self, keycode: KeyCode) {
        if get_time() - self.last_input > 0.1 {
            self.board.keydown(keycode);
            self.last_input = get_time();
        }
    }

    pub fn render(&mut self, window_width: f32, window_heigh: f32) -> () {
        clear_background(WHITE);
        self.board.render(window_width, window_heigh);
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            30.0,
            50.0,
            BLACK,
        );
    }

    pub fn update(&mut self) -> () {
        if get_time() - self.last_update > self.speed {
            self.board.update();
            self.last_update = get_time();
        }
    }
}
