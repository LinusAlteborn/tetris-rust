use std::time::{Instant, Duration};

mod utilities;

pub use utilities::*;
pub mod io;
use io::{output::*, input::*};

const ROWS: usize = 20;
const COLUMNS: usize = 10;

pub struct System {
    pub output: Output,
    pub input: Input,
    pub data: Data,
    pub events: Vec<Gameloop_Events>,
}

pub enum Move{
    Translate(Point),
    Rotate(i32),
    Drop,
}

pub enum Gameloop_Events{
    Respawn,
    Collision,
    Death,
}

impl System {
    pub fn new() -> System {
        System {
            output: Output::new(),
            input: Input::new(),
            data: Data::new(),
            events: vec![],
        }
    }

    pub fn check_move_timer(&mut self) {
        if self.data.move_timer.elapsed() > Duration::from_millis(1000) {
            self.reset_move_timer();
            self.try_move(Move::Translate(Point::from_pos(0.0, 1.0)));
        }
    }

    fn reset_move_timer(&mut self) {
        self.data.move_timer = Instant::now();
    }

    fn new_extent(&self, delta: &Move) -> Vec<Point> {
        let new_extent = match delta {
            Move::Translate(delta) => self.data.shape.to_world_pos(self.data.pos + *delta),
            Move::Rotate(delta) => self.data.shape.rotated(*delta).to_world_pos(self.data.pos),
            Move::Drop => self.data.shape.to_world_pos(self.data.pos + Point::from_pos(0.0, 1.0)),
        };
        new_extent
    }

    fn collision_with_extent(&self, extent: &Vec<Point>) -> bool {
        for point in extent.iter() {
            let x = point.x;
            let y = point.y;
            if x >= COLUMNS as f32 || x < 0.0
            || y >= ROWS as f32 || y < 0.0
            || self.data.grid[y as usize][x as usize] > 0 {
                return true
            }
        }
        false
    }

    pub fn collision(&self, delta: &Move) -> bool {
        self.collision_with_extent(&self.new_extent(delta))
    }

    fn find_rows_with_tetris(&self) -> Vec<usize> {
        let mut rows = Vec::new();
        for point in self.data.shape.to_world_pos(self.data.pos) {
            let y = point.y as usize;
            if !rows.contains(&y) && !self.data.grid[y].contains(&0) {
                rows.push(y);
            }
        }
        rows.sort_unstable();
        rows.reverse();
        rows
    }

    fn check_for_tetris(&mut self) {
        let rows = self.find_rows_with_tetris();
        if rows.len() == 0 {return};
        let mut jump_length = 0;
        for row in (0..=rows[0]).rev() {
            if rows.contains(&row) {
                jump_length += 1;
                continue;
            }
            self.data.grid[row + jump_length] = self.data.grid[row];
        }
        for row in 0..jump_length {
            self.data.grid[row] = [0;COLUMNS];
        }
    }

    pub fn try_move(&mut self, delta: Move) {
        let coll = self.collision(&delta);
        if coll {
            match delta {
                Move::Translate(Point { x: 0.0, y: 1.0 }) | Move::Drop => self.respawn(),
                _ => (),
            }
        } else {
            self.do_move(delta);
        }
    }

    fn stamp(&mut self) {
        for point in self.data.shape.to_world_pos(self.data.pos) {
            let x = point.x as usize;
            let y = point.y as usize;
            self.data.grid[y][x] = self.data.color;
        }
        self.check_for_tetris();
    }

    fn respawn(&mut self) {
        self.stamp();
        self.data.respawn();
        self.reset_move_timer();
        if self.collision(&Move::Translate(Point::from_pos(0.0, 0.0))) {
            self.events.push(Gameloop_Events::Death);
        }
    }

    pub fn do_move(&mut self, delta: Move) {
        match delta {
            Move::Translate(delta) => self.data.pos = self.data.pos +  delta,
            Move::Rotate(delta) => {self.data.shape = self.data.shape.rotated(delta)},
            Move::Drop => {
                self.data.pos = self.data.pos + Point::from_pos(0.0, 1.0);
                self.try_move(Move::Drop);
            }
        }
    }
}

pub struct Data {
    grid: [[usize;COLUMNS];ROWS],
    pos: Point,
    shape: Shape,
    shapes: Vec<Shape>,
    color: usize,
    move_timer: Instant,
}

impl Data {
    fn new() -> Data {
        let shapes = Shape::shapes_from(include_str!("shapes.txt"));
        let mut data = Data {
            grid: [[0;COLUMNS];ROWS],
            pos: Point::from_pos(0.0, 0.0),
            shape: shapes[0].clone(),
            shapes,
            color: 0,
            move_timer: Instant::now(),
        };
        data.respawn();
        data
    }

    fn respawn(&mut self) {
        let mut rng = thread_rng();
        let shape_i = rng.gen_range(0..self.shapes.len());
        self.shape = self.shapes[shape_i].clone();
        self.pos = self.shape.get_spawn_pos();
        self.color = rng.gen_range(1..COLORS.len());
    }
}
