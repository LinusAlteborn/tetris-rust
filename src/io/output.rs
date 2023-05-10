use std::io::stdout;

use crossterm::{execute, terminal, Result};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{SetBackgroundColor, ResetColor, Color, SetForegroundColor};

use crate::*;

pub const COLORS: [Color;7] = [Color::Black, Color::Blue, Color::Cyan, Color::Green, Color::Magenta, Color::Red, Color::Yellow];
const BLOCK_WIDTH: usize = 4;
const BLOCK_HEIGHT: usize = 2;
const BACKGROUND: &'static str = include_str!("background.txt");

enum Instruction {
    MoveTo(usize, usize),
    Color(Color),
    TextColor(Color),
    Print(String),
}

impl Instruction {
    fn perform(&self) -> Result<()> {
        match self {
            Instruction::MoveTo(x, y) => execute!(stdout(), MoveTo(*x as u16, *y as u16)),
            Instruction::Color(color) => execute!(stdout(), SetBackgroundColor(*color)),
            Instruction::TextColor(color) => execute!(stdout(), SetForegroundColor(*color)),
            Instruction::Print(text) => {
                print!("{:.BLOCK_WIDTH$}", text);
                Ok(())
            }
        }
    }
}

pub struct Output {
    grid: [[usize;COLUMNS];ROWS],
    background: [[char;BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS],
    foreground: [[char;BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS],
    offset: (usize, usize),
}

impl Output {
    pub fn new() -> Self {
        let (width, _height) = terminal::size().unwrap();
        let offset = Self::offset(width);
        execute!(stdout(), Hide, MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
        Output {
            grid: [[0;COLUMNS];ROWS],
            background: Self::parse_background(BACKGROUND),
            foreground: [[' ';BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS],
            offset,
        }
    }

    fn parse_background(background_str: &'static str) -> [[char;BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS] {
        let mut background = [[' ';BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS];
        let mut widths = Vec::new();
        let mut height = 0;
        for line in background_str.lines() {
            widths.push(line.len());
            height += 1;
        }

        for (y, line) in background_str.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let width = widths[y];
                let x = x + (COLUMNS * BLOCK_WIDTH - width) / 2;
                let y = y + (ROWS * BLOCK_HEIGHT - height) / 2;
                background[y][x] = char;
            }
        }

        background
    }

    fn changes(&mut self, data: &GameState) -> Vec<(usize, usize)> {
        let next_grid = Self::next_grid(data);
        let mut changed_blocks = Vec::new();
        for x in 0..COLUMNS {
            for y in 0..ROWS {
                if self.grid[y][x] != next_grid[y][x] {
                    changed_blocks.push((x, y));
                }
            }
        }
        self.grid = next_grid;
        changed_blocks
    }

    fn next_grid(data: &GameState) -> [[usize;COLUMNS];ROWS] {
        let mut grid = data.grid;
        if let Some(player) = &data.player {
            for (x, y) in player.extent() {
                let x = x as usize;
                let y = y as usize;
                if x < COLUMNS && y < ROWS {
                    grid[y][x] = player.color;
                }
            }
        }
        grid
    }

    fn color_at(&self, x: usize, y: usize) -> Color {
        let value = self.grid[y][x];
        if value == 0 {
            if (x + y) % 2 == 0 {
                Color::Rgb { r: 15, g: 15, b: 15 }
            } else {
                Color::Rgb { r: 10, g: 10, b: 10 }
            }
        } else {
            COLORS[value]
        }
    }

    fn instructions(&self, changes: Vec<(usize, usize)>) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        instructions.push(Instruction::TextColor(Color::Black));
        for (x, y) in changes {
            instructions.push(Instruction::Color(self.color_at(x, y)));
            for row in 0..BLOCK_HEIGHT {
                instructions.push(Instruction::MoveTo(x * BLOCK_WIDTH + self.offset.0, y * BLOCK_HEIGHT + row + self.offset.1));
                if self.grid[y][x] == 0 {
                    instructions.push(Instruction::Print(Self::compose_back_and_fore(&self.background[y * BLOCK_HEIGHT + row][(x * BLOCK_WIDTH)..(x * BLOCK_WIDTH + BLOCK_WIDTH)], &self.foreground[y * BLOCK_HEIGHT + row][(x * BLOCK_WIDTH)..(x * BLOCK_WIDTH + BLOCK_WIDTH)]).iter().collect()));
                } else {
                    instructions.push(Instruction::Print(self.foreground[y * BLOCK_HEIGHT + row][x * BLOCK_WIDTH..(x * BLOCK_WIDTH + BLOCK_WIDTH)].iter().collect()));
                }
            }
        }
        instructions
    }

    fn compose_back_and_fore(background: &[char], foreground: &[char]) -> [char;BLOCK_WIDTH] {
        let mut composed = [' ';BLOCK_WIDTH];
        for i in 0..BLOCK_WIDTH {
            if foreground[i] == ' ' {
                composed[i] = background[i]
            } else {
                composed[i] = foreground[i]
            }
        }
        composed
    }

    pub fn draw_score(&mut self, score: String) {
        for (x, char) in score.chars().enumerate() {
            let x = (COLUMNS * BLOCK_WIDTH - score.len()) / 2 + x;
            let y = 6;
            self.foreground[y][x] = char;
            self.grid[y / BLOCK_HEIGHT][x / BLOCK_WIDTH] = usize::MAX;
        }
    }
    
    pub fn draw_fps(&mut self, fps: String) {
        for (x, char) in fps.chars().enumerate() {
            let x = x;
            let y = ROWS * BLOCK_HEIGHT - 1;
            self.foreground[y][x] = char;
            self.grid[y / BLOCK_HEIGHT][x / BLOCK_WIDTH] = usize::MAX;
        }
    }

    pub fn draw(&mut self, data: &GameState) {
        let changes = self.changes(data);
        let instructions = self.instructions(changes);
        self.execute(instructions);
    }

    pub fn redraw(&mut self, data: &GameState) {
        self.grid = [[usize::MAX;COLUMNS];ROWS];
        self.draw(data);
    }
    
    fn offset(width: u16) -> (usize, usize) {
        ((width as usize - (COLUMNS * BLOCK_WIDTH + 2)) / 2, 0)
    }

    fn execute(&self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.perform().unwrap();
        }
        execute!(stdout(), ResetColor, Hide, MoveTo((COLUMNS * BLOCK_WIDTH + self.offset.0) as u16, (ROWS * BLOCK_HEIGHT + self.offset.1) as u16)).unwrap();
    }
}
