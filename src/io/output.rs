use std::io::{stdout, Write};

use crossterm::{execute, Result};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::terminal::{SetSize, SetTitle};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};

use crate::*;

pub const COLORS: [Color;7] = [Color::Black, Color::Blue, Color::Cyan, Color::Green, Color::Magenta, Color::Red, Color::Yellow];
pub const BLOCK_WIDTH: usize = 2;

#[derive(Copy, Clone)]
pub struct Cell{
    text: &'static str,
    color: Color,
}

impl Cell {
    fn from_color(color: Color) -> Cell {
        Cell {
            text: "   ",
            color,
        }
    }
}

pub struct Output {
    cells: [[Cell;COLUMNS];ROWS],
}

impl Output {
    pub fn new() -> Output {
        execute!(stdout(), Hide, SetForegroundColor(Color::Blue)).unwrap();
        Output {
            cells: [[Cell::from_color(COLORS[0]);COLUMNS];ROWS],
        }
    }

    pub fn update(&mut self, data: &Data) {
        for (y, row) in data.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.cells[y][x] = Cell::from_color(COLORS[*cell])
            }
        }

        let extent = &data.shape.to_world_pos(data.pos);
        for point in extent {
            let x = point.x as usize;
            let y = point.y as usize;
            if x < COLUMNS && y < ROWS {
                self.cells[y][x] = Cell::from_color(COLORS[data.color]);
            }
        }

        self.draw();
    }
    
    fn draw(&self) {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                execute!(stdout(), MoveTo(col as u16 * BLOCK_WIDTH as u16, row as u16), SetBackgroundColor(self.cells[row][col].color)).unwrap();
                print!("{:.2}", self.cells[row][col].text);
            }
        }
        execute!(stdout(), ResetColor).unwrap();
    }
}