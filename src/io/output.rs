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

/// Detta är en representation av kommandon som skickas till terminalen
/// 
/// Varje typ av denna enum representerar ett execute!() kommand. och typerna i dessa (String, usize...) är anpassade fär att vara lättanvände i mitt syfte.
enum Instruction {
    MoveTo(usize, usize),
    Color(Color),
    TextColor(Color),
    Print(String),
}

impl Instruction {
    /// Denna funktion utför en instruktion och ger tillbaka (returnar) med antingen ett Ok() som betyder att den lyckades eller ett std::io::Error med ett meddelande om varför det inte fungerade.
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

/// Denna struct samlar data för dem visuella delarna och hanterar interaktionen med terminalen
/// 
/// Den har även passande hjälpfunktioner för att manipulera denna data. Detta är mesta dels för att optimisera spelet.
/// 
/// Att göra många execute!() calls är dyrt för prestanda. därför håller vi endast koll på förändringar, alltså vi spara hur spelet ser ut och ser vilka block som har förändrats. Sedan målar vi endast dom blocken. Detta är varför vi behöver data i denna struct, för att se förändringar.
pub struct Output {
    grid: [[usize;COLUMNS];ROWS],
    background: [[char;BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS],
    foreground: [[char;BLOCK_WIDTH * COLUMNS];BLOCK_HEIGHT * ROWS],
    offset: (usize, usize),
}

impl Output {
    /// Denna funktion skapar ett nytt output instans med grund värden
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

    /// Denna funktion läser filen background.txt och formatterar denna för att se fin ut. D.V.S. Vi centrerar texten och delar upp den i block för att vara lättare att jobba med.
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

    /// Denna funktion jämför grid hos output och gamestate och hittar vilka block positioner som har ändrade värden.
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

    /// Denna funktion tar data över grid och spelare för att bestämma vilka värden output grid skal ha. Spelaren syns inte om man inte gör detta då den inte är en del av gamestatets grid förräns den placerats.
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

    /// Denna funktion tar ett x och y värde och bestämmer vilken färg denna cell skal ha. Denna är ansvarig för rutnätet som bakgrunden har.
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

    /// Denna funktionen tar en vector över alla celler som ändrats konverterar detta till instruktioner som terminalen skal utföra.
    fn instructions(&self, changes: Vec<(usize, usize)>) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        instructions.push(Instruction::TextColor(Color::Grey));
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

    /// Denna funktion kombinderar bakgrund och ui. Den ser till att UI alltid ligger över bakgrunder.
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

    /// Denna funktion manipulerar foreground fältet för att skriva text med spelarens poäng. Den beräknar positionen i 2D arrayen som den skal ändra för att det skal bli korrekt.
    pub fn draw_score(&mut self, score: String) {
        for (x, char) in score.chars().enumerate() {
            let x = (COLUMNS * BLOCK_WIDTH - score.len()) / 2 + x;
            let y = 6;
            self.foreground[y][x] = char;
            self.grid[y / BLOCK_HEIGHT][x / BLOCK_WIDTH] = usize::MAX;
        }
    }
    
    /// Denna funktion är samma som draw_score fast den sitter i nedre vänstra hörnet. Kan vara bra att ha en generel funktion eftersom att dessa funktioner är väldigt lika.
    pub fn draw_fps(&mut self, fps: String) {
        for (x, char) in fps.chars().enumerate() {
            let x = x;
            let y = ROWS * BLOCK_HEIGHT - 1;
            self.foreground[y][x] = char;
            self.grid[y / BLOCK_HEIGHT][x / BLOCK_WIDTH] = usize::MAX;
        }
    }

    /// Denna funktion hitta förändringar, skapar instructioner för dessa och utför dem. D.V.S. den updaterar block som förändrats.
    pub fn draw(&mut self, data: &GameState) {
        let changes = self.changes(data);
        let instructions = self.instructions(changes);
        self.execute(instructions);
    }

    /// Denna funktionen målar om alla block. Till skillnad från draw() så kollar den inte efter skillnad, utan målar om allting. Denna är användbar om något glitchat, t.ex. om man gör fönstret för litet och spelet buggar ut.
    pub fn redraw(&mut self, data: &GameState) {
        self.grid = [[usize::MAX;COLUMNS];ROWS];
        self.draw(data);
    }
    
    /// Denna funktion använder bredden på skärmen för att beräkna ett offset så allting hamnar i mitten på skärmen.
    fn offset(width: u16) -> (usize, usize) {
        ((width as usize - (COLUMNS * BLOCK_WIDTH + 2)) / 2, 0)
    }

    /// Denna funktion itererar genom en vector av instructioner och utför dessa. Den ställer tillbaka terminalen till dess grundvärden efter den är färdig.
    fn execute(&self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            instruction.perform().unwrap();
        }
        execute!(stdout(), ResetColor, Hide, MoveTo((COLUMNS * BLOCK_WIDTH + self.offset.0) as u16, (ROWS * BLOCK_HEIGHT + self.offset.1) as u16)).unwrap();
    }
}
