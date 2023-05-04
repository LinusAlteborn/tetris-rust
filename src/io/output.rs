use std::io::stdout;

use crossterm::{
    execute,
    cursor::{Hide, MoveTo},
};

use crate::*;

pub const COLORS: [&str;7] = ["  ","🟥","🟦","🟧","🟨","🟩","🟪"];

pub struct Output {
    cells: [[&'static str;COLUMNS];ROWS],
}

impl Output {
    pub fn new() -> Output {
        execute!(stdout(), Hide).unwrap();
        Output {
            cells: [["  ";COLUMNS];ROWS],
        }
    }

    pub fn update(&mut self, data: &Data) {
        for (y, row) in data.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.cells[y][x] = match cell {
                    0 => "  ",
                    color => COLORS[*color],
                }
            }
        }

        let extent = &data.shape.to_world_pos(data.pos);
        for point in extent {
            let x = point.x as usize;
            let y = point.y as usize;
            if x < COLUMNS && y < ROWS {
                self.cells[y][x] = COLORS[data.color];
            }
        }

        self.draw();
    }
    
    fn draw(&self) {
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        let mut out = String::from("");
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                out.push_str(self.cells[row][col]);
            }
            out.push('\n');
        }
        print!("{out}");
    }
}