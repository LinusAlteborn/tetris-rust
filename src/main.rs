use std::time::Duration;
use std::io::{stdout, Write};

use crossterm::ExecutableCommand;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode, KeyEventKind},
    cursor::{MoveTo, Hide},
    terminal::{ScrollUp},
};

const COLOR: [char; 9] = ['⬛','⬜','🟥','🟦','🟧','🟨','🟩','🟪','🟫'];
const BACKGROUND: &str = "\
Tetris - Rust
-----------------
Carl, Tore, Linus\
";
const ROWS: usize = 20;
const COLS: usize = 20;
const SHAPES: [&'static str;3] = [
"\
###
#..
","\
##
##
","\
###
.#.
",
];

struct UI {
    background: [[char;COLS * 2];ROWS],
}

impl UI {
    fn new() -> UI {
        UI {
            background: UI::format_background(),
        }
    }

    fn format_background() -> [[char;COLS * 2];ROWS]{
        let mut out = [[' ';COLS * 2];ROWS];
        for (row, line) in BACKGROUND.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                out[row + (ROWS - BACKGROUND.lines().count()) / 2][col + (COLS * 2 - line.len()) / 2] = char;
            }
        }
        out
    }
}

struct Interface {
    stdout: std::io::Stdout,
}

impl Interface {
    fn new() -> Interface {
        let mut stdout = stdout();
        stdout.execute(Hide).unwrap();
        stdout.execute(ScrollUp(999)).unwrap();
        Interface {
            stdout,
        }
    }
    
    fn input<F: FnOnce(KeyCode) -> Game_Event>(&mut self, state: &mut State, key_match: F) {
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(KeyEvent{kind: KeyEventKind::Press, code, ..}) = read().unwrap() {
                state.add_event(key_match(code));
            }
        }
    }

    fn render(&mut self, state: &State, ui: &UI) {
        self.stdout.execute(MoveTo(0, 0)).unwrap();
        let mut buffer = String::from("");
        for row in 0..ROWS {
            for col in 0..COLS {
                let color = state.grid[row][col];
                if state.block.is_block(col, row) {
                    buffer.push(COLOR[state.block.color]);
                } else if color == 0 {
                    buffer.push(ui.background[row][col * 2]);
                    buffer.push(ui.background[row][col * 2 + 1]);
                } else {
                    buffer.push(COLOR[color]);
                };
            }
            buffer.push('\n');
        }
        print!("{buffer}");
        println!("{}, {}", state.block.x, state.block.y);
        self.stdout.flush().unwrap();
    }
}

#[derive(Debug)]
struct Block {
    x: i32,
    y: i32,
    rot: i32,
    color: usize,
    shape: Vec<Vec<bool>>,
    height: i32,
    width: i32,

}

impl Block {
    fn new() -> Block {
        let shape = 0;
        let (height, width, shape) = Block::shape(shape);
        Block {
            x: 0,
            y: 0,
            rot: 0,
            color: 1,
            shape,
            height,
            width,
        }
    }

    fn shape(shape: usize) -> (i32, i32, Vec<Vec<bool>>) {
        let mut out = vec![];
        for row in SHAPES[shape].lines(){
            let mut line = vec![];
            for b in row.bytes() {
                line.push(b == b'#')
            }
            out.push(line);
        }
        (out.len().try_into().unwrap(), out[0].len().try_into().unwrap(), out)
    }

    fn is_block(&self, x: usize, y: usize) -> bool {
        let (x, y) = (x as i32 - self.x, y as i32 - self.y);
        if (x < 0 || x >= self.width) || (y < 0 || y >= self.height) {
            return false
        }
        self.shape[y as usize][x as usize]
    }

    fn wall_collision(&self, x: i32, y: i32) -> bool {
        let x = self.x + x;
        let y = self.y + y;
        x < 0 || y < 0 || x + self.width > COLS as i32 || y + self.height > ROWS as i32
    }

    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn rotate(&mut self, r: i32) {
        self.rot = r;
        self.rot %= 4;
    }
}

#[derive(Debug)]
enum Game_Event{
    Move(i32, i32),
    Rotate(i32),
    Drop,
    Quit,
}

struct State {
    grid: [[usize;COLS];ROWS],
    block: Block,
    events: Vec<Game_Event>,
}

impl State {
    fn new() -> State {
        State {
            grid: [[0;COLS];ROWS],
            block: Block::new(),
            events: vec![],
        }
    }

    pub fn process_events(&mut self) {
        for _ in 0..self.events.len() {
            match self.events.pop().unwrap() {
                Game_Event::Move(x, y) => self.move_block(x, y),
                Game_Event::Rotate(r) => self.rotate_block(r),
                Game_Event::Quit => std::process::exit(0),
                Game_Event::Drop => self.drop_block(),
            }
        }
    }

    pub fn add_event(&mut self, event: Game_Event) {
        self.events.push(event)
    }
}

fn main() {
    // for handling io. input (key events) and output (cursor/print).
    let mut io = Interface::new();
    // for handling the game logic
    let mut state = State::new();
    // for easier use of Graphical stuffs
    let mut ui = UI::new();
    // loop
    'game_loop : loop {
        io.input(
            &mut state,
            |code| match code {
            KeyCode::Char(' ') | KeyCode::Backspace => Game_Event::Drop,
            KeyCode::Char('w') | KeyCode::Up => Game_Event::Move(0,-1),
            KeyCode::Char('a') | KeyCode::Left => Game_Event::Move(-1,0),
            KeyCode::Char('s') | KeyCode::Down => Game_Event::Move(0,1),
            KeyCode::Char('d') | KeyCode::Right => Game_Event::Move(1,0),
            KeyCode::Char('r') | KeyCode::BackTab => Game_Event::Rotate(1),
            KeyCode::Char('q') | KeyCode::Esc => Game_Event::Quit,
            _ => todo!(),
            }
        );
        state.process_events();
        io.render(&state, &ui);
    }
}
