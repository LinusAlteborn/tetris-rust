use std::collections::HashMap;
use std::io::stdout;

use crossterm::cursor::{Hide, MoveTo};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, SetSize};
use crossterm::{execute, terminal, Result};

use crate::*;

pub const COLORS: [Color; 7] = [
    Color::Black,
    Color::Blue,
    Color::Cyan,
    Color::Green,
    Color::Magenta,
    Color::Red,
    Color::Yellow,
];
pub const BLOCK_WIDTH: usize = 4;
pub const BLOCK_HEIGHT: usize = 2;

#[derive(Copy, Clone)]
pub struct Cell {
    text: [&'static str; BLOCK_HEIGHT],
    color: Color,
}

impl Cell {
    fn from_color(color: Color) -> Cell {
        Cell {
            text: if color == Color::Black {
                ["      "; BLOCK_HEIGHT]
            } else {
                ["      "; BLOCK_HEIGHT]
            },
            color,
        }
    }
}

#[derive(PartialEq, Eq)]
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
            }
        }
    }
}

pub struct Output {
    cells: [[Cell; COLUMNS]; ROWS],
    cells_mem: [[Cell; COLUMNS]; ROWS],
    offset: usize,
    instant: Instant,
    instructions: Vec<Instruction>,
}

impl Output {
    pub fn new() -> Output {
        let (width, height) = Self::min_size();
        let offset = Self::offset(width);
        execute!(
            stdout(),
            Hide,
            MoveTo(0, 0),
            Clear(ClearType::FromCursorDown),
            SetSize(width, height)
        )
        .unwrap();
        Output {
            cells: [[Cell::from_color(COLORS[0]); COLUMNS]; ROWS],
            cells_mem: [[Cell::from_color(COLORS[1]); COLUMNS]; ROWS],
            offset,
            instant: Instant::now(),
            instructions: vec![],
        }
    }

    fn check_for_full_update(&mut self) {
        if self.instant.elapsed().as_millis() > 1500 {
            let instructions_by_color = self.instruction_by_color();
            self.fill_instructions(instructions_by_color);
            self.instant = Instant::now();
        }
    }

    fn update_cells(&mut self, data: &Data) {
        self.cells_mem = self.cells;
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
                    instructions.push(Instruction::MoveTo(
                        x * BLOCK_WIDTH + 1 + self.offset,
                        y * BLOCK_HEIGHT + row + 1,
                    ));
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
            if color == COLORS[0] {
                for y in 1..ROWS * BLOCK_HEIGHT + 1 {
                    self.instructions.push(Instruction::MoveTo(
                        COLUMNS * BLOCK_WIDTH + 1 + self.offset,
                        y,
                    ));
                    self.instructions.push(Instruction::Print("|"));
                    self.instructions
                        .push(Instruction::MoveTo(0 + self.offset, y));
                    self.instructions.push(Instruction::Print("|"));
                }
                for x in 0..COLUMNS {
                    self.instructions
                        .push(Instruction::MoveTo(x * BLOCK_WIDTH + 1 + self.offset, 0));
                    self.instructions.push(Instruction::Print("----------"));
                    self.instructions.push(Instruction::MoveTo(
                        x * BLOCK_WIDTH + 1 + self.offset,
                        ROWS * BLOCK_HEIGHT + 1,
                    ));
                    self.instructions.push(Instruction::Print("----------"));
                }
            }
        }
    }

    fn cell_changes(&self) -> Vec<(usize, usize)> {
        let mut changes = Vec::new();
        for y in 0..ROWS {
            for x in 0..COLUMNS {
                if self.cells[y][x].color != self.cells_mem[y][x].color {
                    changes.push((x, y));
                }
            }
        }
        changes
    }

    fn changes_to_instructions(&self, changes: Vec<(usize, usize)>) -> Vec<Instruction> {
        let mut instructions_by_color = HashMap::new();
        for (x, y) in changes {
            let cell = self.cells[y][x];
            let instructions = instructions_by_color
                .entry(cell.color)
                .or_insert(vec![Instruction::ChangeColor(cell.color)]);
            for row in 0..BLOCK_HEIGHT {
                instructions.push(Instruction::MoveTo(
                    x * BLOCK_WIDTH + 1 + self.offset,
                    y * BLOCK_HEIGHT + row + 1,
                ));
                instructions.push(Instruction::Print(cell.text[row]));
            }
        }

        let mut out = Vec::new();
        for (color, instructions) in instructions_by_color {
            out.push(Instruction::ChangeColor(color));
            for instruction in instructions {
                out.push(instruction);
            }
        }
        out.push(Instruction::ChangeColor(COLORS[0]));
        for y in 1..ROWS * BLOCK_HEIGHT + 1 {
            out.push(Instruction::MoveTo(
                COLUMNS * BLOCK_WIDTH + 1 + self.offset,
                y,
            ));
            out.push(Instruction::Print("|"));
            out.push(Instruction::MoveTo(0 + self.offset, y));
            out.push(Instruction::Print("|"));
        }
        for x in 0..COLUMNS {
            out.push(Instruction::MoveTo(x * BLOCK_WIDTH + 1 + self.offset, 0));
            out.push(Instruction::Print("----------"));
            out.push(Instruction::MoveTo(
                x * BLOCK_WIDTH + 1 + self.offset,
                ROWS * BLOCK_HEIGHT + 1,
            ));
            out.push(Instruction::Print("----------"));
        }

        out
    }

    pub fn update(&mut self, data: &Data) {
        self.update_cells(data);
        let changes = self.cell_changes();
        let new_instrctions = self.changes_to_instructions(changes);
        self.instructions = new_instrctions;

        self.check_for_full_update();

        self.use_instructions();
    }

    fn min_size() -> (u16, u16) {
        let (width, height) = terminal::size().unwrap();
        let min_width = (COLUMNS * BLOCK_WIDTH + 2) as u16;
        let min_height = (ROWS * BLOCK_HEIGHT + 3) as u16;
        let width = if width > min_width { width } else { min_width };
        let height = if height > min_height {
            height
        } else {
            min_height
        };
        (width, height)
    }

    fn offset(width: u16) -> usize {
        (width as usize - (COLUMNS * BLOCK_WIDTH + 2)) / 2
    }

    fn use_instructions(&mut self) {
        execute!(stdout(), SetForegroundColor(Color::DarkGrey), Hide).unwrap();
        for instruction in self.instructions.drain(..) {
            instruction.perform().unwrap();
        }
        execute!(
            stdout(),
            ResetColor,
            Hide,
            MoveTo(
                (COLUMNS * BLOCK_WIDTH + 2 + self.offset) as u16,
                (ROWS * BLOCK_HEIGHT + 1) as u16
            )
        )
        .unwrap();
    }
}
