use std::io::stdout;
use std::collections::HashMap;

use crossterm::{execute, Result};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{SetForegroundColor, SetBackgroundColor, ResetColor, Color};

use crate::*;

pub const COLORS: [Color;7] = [Color::Black, Color::Blue, Color::Cyan, Color::Green, Color::Magenta, Color::Red, Color::Yellow];
pub const BLOCK_WIDTH: usize = 4;
pub const BLOCK_HEIGHT: usize = 2;

#[derive(Copy, Clone)]
pub struct Cell{
    text: [&'static str;BLOCK_HEIGHT],
    color: Color,
}

impl Cell {
    fn from_color(color: Color) -> Cell {
        Cell {
            text: ["     ";BLOCK_HEIGHT],
            color,
        }
    }
}

enum Instruction {
    MoveTo(usize, usize),
    ChangeColor(Color),
    Print(&'static str),
}

impl Instruction {
    fn perform(&self) -> Result<()> {
        match self {
            Instruction::MoveTo(x, y) => execute!(stdout(), MoveTo(*x as u16, *y as u16)),
            Instruction::ChangeColor(color) => execute!(stdout(), SetBackgroundColor(*color)),
            Instruction::Print(text) => {
                print!("{:.BLOCK_WIDTH$}", text);
                Ok(())
            },
        }
    }
}

pub struct Output {
    cells: [[Cell;COLUMNS];ROWS],
    instructions: Vec<Instruction>,
}

impl Output {
    pub fn new() -> Output {
        execute!(stdout(), Hide, SetForegroundColor(Color::Blue), Clear(ClearType::FromCursorDown)).unwrap();
        Output {
            cells: [[Cell::from_color(COLORS[0]);COLUMNS];ROWS],
            instructions: vec![],
        }
    }

    fn update_cells(&mut self, data: &Data) {
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
    }

    fn instruction_by_color(&self) -> HashMap<Color, Vec<Instruction>> {
        let mut instruction_by_color: HashMap<Color, Vec<Instruction>> = HashMap::new();
        
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let instructions = instruction_by_color.entry(cell.color).or_insert(vec![]);
                for row in 0..BLOCK_HEIGHT {
                    instructions.push(Instruction::MoveTo(x * BLOCK_WIDTH, y * BLOCK_HEIGHT + row));
                    instructions.push(Instruction::Print(cell.text[row]));
                }
            }
        }

        instruction_by_color
    }

    fn fill_instructions(&mut self, mut instructions_by_color: HashMap<Color, Vec<Instruction>>) {
        for (color, instructions) in instructions_by_color.drain() {
            self.instructions.push(Instruction::ChangeColor(color));
            for instruction in instructions {
                self.instructions.push(instruction);
            }
        }
    }

    pub fn update(&mut self, data: &Data) {
        self.update_cells(data);
        let instructions_by_color = self.instruction_by_color();
        self.fill_instructions(instructions_by_color);

        self.use_instructions();
    }
    
    fn use_instructions(&mut self) {
        for instruction in self.instructions.drain(..) {
            instruction.perform().unwrap();
        }
        execute!(stdout(), ResetColor, MoveTo((COLUMNS * BLOCK_WIDTH) as u16, (ROWS * BLOCK_HEIGHT) as u16)).unwrap();
    }
}