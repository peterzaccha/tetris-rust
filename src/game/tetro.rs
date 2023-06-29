use std::f32::consts::PI;

use macroquad::{
    color::{Color, BLUE, DARKBLUE, GREEN, ORANGE, PURPLE, RED, YELLOW},
    shapes::draw_rectangle,
};

#[derive(Clone, Copy)]
pub enum TetroKind {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}
pub struct Tetro {
    kind: TetroKind,
    pub cells: Vec<Vec<bool>>,
    origin: (f32, f32),
    pub color: Color,
}

impl Tetro {
    pub fn new(kind: TetroKind) -> Self {
        match kind {
            TetroKind::I => Self {
                kind,
                cells: vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                origin: (1.5, 1.5),
                color: BLUE,
            },
            TetroKind::O => Self {
                kind,
                cells: vec![vec![true, true], vec![true, true]],
                origin: (0.5, 0.5),
                color: YELLOW,
            },
            TetroKind::T => Self {
                kind,
                cells: vec![
                    vec![true, true, true],
                    vec![false, true, false],
                    vec![false, false, false],
                ],
                origin: (1.0, 1.0),
                color: PURPLE,
            },
            TetroKind::J => Self {
                kind,
                cells: vec![
                    vec![false, true, false],
                    vec![false, true, false],
                    vec![true, true, false],
                ],
                origin: (1.0, 1.0),
                color: DARKBLUE,
            },
            TetroKind::L => Self {
                kind,
                cells: vec![
                    vec![false, true, false],
                    vec![false, true, false],
                    vec![false, true, true],
                ],
                origin: (1.0, 1.0),
                color: ORANGE,
            },
            TetroKind::S => Self {
                kind,
                cells: vec![
                    vec![false, true, true],
                    vec![true, true, false],
                    vec![false, false, false],
                ],
                origin: (1.0, 1.0),
                color: GREEN,
            },
            TetroKind::Z => Self {
                kind,
                cells: vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![false, false, false],
                ],
                origin: (1.0, 1.0),
                color: RED,
            },
        }
    }

    pub fn width(&self) -> usize {
        return self
            .cells
            .iter()
            .map(|a| a.iter().map(|&b| b as usize).sum())
            .max()
            .unwrap();
    }
    pub fn hieght(&self) -> usize {
        return self
            .cells
            .iter()
            .filter(|&a| a.iter().all(|&b| b))
            .collect::<Vec<_>>()
            .len();
    }

    pub fn get_cells_in_board(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut cells: Vec<(usize, usize)> = vec![];
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let new_x: i8 = x as i8 + position.0 as i8;
                let new_y: i8 = y as i8 + position.1 as i8;
                if *cell {
                    cells.push((new_x as usize, new_y as usize));
                }
            }
        }
        return cells;
    }

    pub fn render(
        &self,
        position: (usize, usize),
        cell_size: f32,
        start_x: f32,
        start_y: f32,
    ) -> () {
        for (x, y) in self.get_cells_in_board(position) {
            draw_rectangle(
                x as f32 * cell_size + start_x,
                y as f32 * cell_size + start_y,
                cell_size,
                cell_size,
                self.color,
            );
        }
    }

    pub fn rotate(&mut self) -> () {
        let angle: f32 = PI / 2.0;
        let new_width = self.cells[0].len();
        let new_height = self.cells.len();

        let mut new_shape: Vec<Vec<bool>> = vec![vec![false; new_width]; new_height];

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    let ox = x as f32 - self.origin.0;
                    let oy = y as f32 - self.origin.1;
                    let new_x =
                        (ox as f32 * angle.cos()) - (oy as f32 * angle.sin()) + self.origin.0;
                    let new_y =
                        (ox as f32 * angle.sin()) + (oy as f32 * angle.cos()) + self.origin.1;
                    new_shape[new_y.round() as usize][new_x.round() as usize] = *cell
                }
            }
        }
        self.cells = new_shape;
    }
}
