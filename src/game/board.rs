use std::cmp::{self, Ordering};
use std::{collections::VecDeque, usize};

use macroquad::prelude::Vec2;
use macroquad::ui::root_ui;
use macroquad::{
    color::{Color, BLACK, BLUE, RED},
    input::KeyCode,
    rand::gen_range,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use super::tetro::{Tetro, TetroKind};

const BOARD_WIDTH: usize = 10;
const BOARD_HIGHT: usize = 20;

pub const CELL_SIZE: usize = 30;

enum GameStatus {
    Running,
    Over,
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    Filled(Color),
}
pub struct Board {
    grid: VecDeque<Vec<Cell>>,
    current_tetro: Tetro,
    current_tetro_position: (usize, usize),
    status: GameStatus,
}

pub enum MoveOutcome {
    Valid,
    Lock,
    Prevent,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: VecDeque::from(vec![vec![Cell::Empty; BOARD_WIDTH]; BOARD_HIGHT]),
            current_tetro: Tetro::new(TetroKind::O),
            current_tetro_position: (4, 0),
            status: GameStatus::Running,
        }
    }

    pub fn keydown(&mut self, keycode: KeyCode) -> () {
        if let GameStatus::Running = self.status {
            match keycode {
                KeyCode::Left => self.move_current((-1, 0)),
                KeyCode::Right => self.move_current((1, 0)),
                KeyCode::Down => self.move_current((0, 1)),
                KeyCode::Space => self.current_tetro.rotate(),
                _ => (),
            }
        }
    }

    pub fn move_current(&mut self, delta: (i8, i8)) -> () {
        let mut nx = self.current_tetro_position.0 as i8 + delta.0;
        let mut ny = self.current_tetro_position.1 as i8 + delta.1;
        // nx = nx.clamp(
        //     -1 - self.current_tetro.width() as i8,
        //     (BOARD_WIDTH - self.current_tetro.width()) as i8,
        // );
        // ny = ny.clamp(0, (BOARD_HIGHT - 1) as i8);

        // if nx < 0 || nx as usize >= BOARD_WIDTH {
        //     return;
        // }
        // dbg!((&nx, &ny));

        match self.valid_move(
            self.current_tetro
                .get_cells_in_board((nx as usize, ny as usize)),
            delta.0 != 0,
        ) {
            MoveOutcome::Valid => {
                self.current_tetro_position = (nx as usize, ny as usize);
            }
            MoveOutcome::Lock => {
                for position in self
                    .current_tetro
                    .get_cells_in_board(self.current_tetro_position)
                {
                    self.grid[position.1][position.0] = Cell::Filled(self.current_tetro.color);
                }
                self.generate_new_tetro();
            }
            MoveOutcome::Prevent => {}
        }
    }

    fn generate_new_tetro(&mut self) -> () {
        let available = [
            TetroKind::I,
            TetroKind::J,
            TetroKind::L,
            TetroKind::O,
            TetroKind::S,
            TetroKind::T,
            TetroKind::Z,
        ];
        let tetro = Tetro::new(available[gen_range(0, available.len())]);
        self.current_tetro = tetro;
        // self.current_tetro = Tetro::new(TetroKind::O);
        self.current_tetro_position = (4, 0);
    }

    pub fn update(&mut self) -> () {
        if let GameStatus::Running = self.status {
            dbg!("Running");
            self.move_current((0, 1));
            self.check_row();
            self.check_lose();
        }
    }

    fn check_lose(&mut self) -> () {
        dbg!(&self.grid[0]);
        if self.grid[0].iter().any(|a| {
            if let Cell::Filled(_) = a {
                return true;
            }
            false
        }) {
            self.status = GameStatus::Over;
        }
    }

    fn check_row(&mut self) -> () {
        let mut shifts: Vec<usize> = vec![];
        for (i, row) in self.grid.iter().enumerate() {
            if row
                .iter()
                .all(|element| matches!(*element, Cell::Filled(_)))
            {
                shifts.push(i)
            }
        }
        for i in shifts {
            self.grid.remove(i);
            self.grid.push_front(vec![Cell::Empty; BOARD_WIDTH])
        }
    }

    pub fn valid_move(&self, positions: Vec<(usize, usize)>, h_move: bool) -> MoveOutcome {
        for position in positions {
            if position.0 >= BOARD_WIDTH {
                return MoveOutcome::Prevent;
            }

            if position.1 >= BOARD_HIGHT {
                return MoveOutcome::Lock;
            }

            if let Cell::Filled(_) = self.grid[position.1][position.0] {
                if h_move {
                    return MoveOutcome::Prevent;
                }
                return MoveOutcome::Lock;
            }
        }
        MoveOutcome::Valid
    }
    pub fn render(&mut self, window_width: f32, window_heigh: f32) -> () {
        let mut cell_size = 0.0;
        cell_size = window_heigh / (BOARD_HIGHT as f32 + 10.0);
        let start_x = (window_width / 2.0) - (BOARD_WIDTH as f32 * cell_size) / 2.0;
        let start_y = cell_size * 2.0;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => {
                        draw_rectangle_lines(
                            x as f32 * cell_size + start_x,
                            y as f32 * cell_size + start_y,
                            cell_size as f32,
                            cell_size as f32,
                            2.0,
                            BLACK,
                        );
                    }
                    Cell::Filled(c) => {
                        let mut darkc = c.clone();
                        darkc.r -= 0.2;
                        darkc.b -= 0.2;
                        darkc.g -= 0.2;
                        draw_rectangle(
                            x as f32 * cell_size + start_x,
                            y as f32 * cell_size + start_y,
                            cell_size as f32,
                            cell_size as f32,
                            *c,
                        );
                        draw_rectangle_lines(
                            x as f32 * cell_size + start_x,
                            y as f32 * cell_size + start_y,
                            cell_size as f32,
                            cell_size as f32,
                            4.0,
                            darkc,
                        );
                    }
                }
            }
        }

        let ui_y = start_y + BOARD_HIGHT as f32 * cell_size + 10.0;
        if root_ui().button(
            Some(Vec2::from([
                start_x + BOARD_WIDTH as f32 * cell_size / 2.0,
                ui_y,
            ])),
            "Rotate",
        ) {
            self.keydown(KeyCode::Space);
        }
        if root_ui().button(Some(Vec2::from([start_x, ui_y + 30.0])), "Left") {
            self.keydown(KeyCode::Left);
        }
        if root_ui().button(
            Some(Vec2::from([
                start_x + BOARD_WIDTH as f32 * cell_size,
                ui_y + 30.0,
            ])),
            "Right",
        ) {
            self.keydown(KeyCode::Right);
        }
        if root_ui().button(
            Some(Vec2::from([
                start_x + BOARD_WIDTH as f32 * cell_size / 2.0,
                ui_y + 60.0,
            ])),
            "Down",
        ) {
            self.keydown(KeyCode::Down);
        }

        self.current_tetro
            .render(self.current_tetro_position, cell_size, start_x, start_y);
        if let GameStatus::Over = self.status {
            draw_text("Game Over", 10., 100., 100.0, BLACK);
        }
    }
}
