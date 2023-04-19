const COLORS: [char; 9] = ['⬛','🟦','🟨','🟪','🟩','🟥','🟫','🟧','⬜'];
const BACKGROUND: [&str; 1] = [
    "NTI Moment",
];
const ROWS: usize = 20;
const COLUMNS: usize = 10;

use std::io::{stdout, Write};
use crossterm::{
    cursor, terminal, Result, ExecutableCommand
};
use device_query::{DeviceQuery, DeviceState, Keycode};

struct Grid {
    blocks: [[usize;COLUMNS];ROWS],
    stdout: std::io::Stdout,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            blocks: [[0;COLUMNS];ROWS],
            stdout: stdout(),
        }
    }

    fn change(&mut self, x: usize, y: usize, color: usize) {
        self.blocks[y][x] = color;
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.blocks[y][x]
    }

    fn draw(&mut self) {
        for y in 0..ROWS{
            let mut line = String::from("");
            for x in 0..COLUMNS{
                let color = self.blocks[y][x];
                line.push(COLORS[color]);
            }
            writeln!(self.stdout, "{line}").unwrap();
        }
        self.stdout.execute(cursor::MoveUp(ROWS as u16)).unwrap();
    }
}

struct Point(i32, i32);

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

struct Block {
    p: Point,
    r: usize,
    shape: usize,
}

impl Block {
    fn new(shape: usize) -> Block {
        Block {
            p: Point(5, 5),
            r: 0,
            shape: 0,
        }
    }
}

fn main() {
    use std::time::Instant;
    
    let mut grid = Grid::new();
    let mut block = Block::new(0);
    
    let mut time = Instant::now();

    let device_state = DeviceState::new();
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::A) {println!("Pressed A");}
        if time.elapsed().as_millis() > 1000 {
            // get newpos
            // check for collision
            // if serious collision, draw block to grid and spawn new block
            // if not serious, don't move and start again
            grid.draw();
            time = Instant::now();
        }
    }
}
