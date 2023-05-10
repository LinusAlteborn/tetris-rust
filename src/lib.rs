pub use std::time::{Duration, Instant};

mod menu;
mod utilities;
use io::output::COLORS;

pub use menu::*;
pub use utilities::*;

pub mod io;

pub const ROWS: usize = 20;
pub const COLUMNS: usize = 16;
const SHAPES: &'static str = "";

pub struct GameState {
    grid: [[usize;COLUMNS];ROWS],
    player: Option<Player>,
    shapes: Vec<Shape>,
    shape_order: Vec<usize>,
    pub points: usize,
}

impl GameState {
    pub fn new() -> Self {
        let shapes = Shape::parse_shapes(SHAPES);
        Self {
            grid: [[0;COLUMNS];ROWS],
            player: None,
            shapes,
            shape_order: vec![3,2,2,1,0],
            points: 0,
        }
    }

    pub fn give_points(&mut self, rows_cleared: usize) {
        self.points += 100 * 2usize.pow(rows_cleared as u32);
    }

    fn fill_shape_order(&mut self) {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (0..self.shapes.len().clone()).collect();
        nums.shuffle(&mut rng);
        self.shape_order = nums;
    }

    fn next_shape_index(&mut self) -> usize {
        match self.shape_order.pop() {
            Some(shape) => shape,
            None => {
                self.fill_shape_order();
                self.next_shape_index()
            },
        }
    }

    pub fn spawn(&mut self) {
        let shape_index = self.next_shape_index();
        let shape = self.shapes[shape_index].clone();
        let (x, y) = ((COLUMNS as f32 / 2.0 - shape.get_offset().0) as i32, (0.0) as i32);
        let color = thread_rng().gen_range(1..COLORS.len());
        self.player = Some(Player::spawn(x, y, shape, color));
    }

    fn collision(&self) -> Option<Collision> {
        if let Some(player) = &self.player {
            for (x, y) in player.extent() {
                if x < 0 { return Some(Collision::Wall) }
                if x >= COLUMNS as i32 { return Some(Collision::Wall) }
                if y >= ROWS as i32 { return Some(Collision::Floor) }
                if y < 0 { continue; }
                let x = x as usize;
                let y = y as usize;
                if self.grid[y][x] != 0 { return Some(Collision::Block); }
            }
            None
        } else {
            None
        }
    }

    fn do_move(&mut self, player_move: &PlayerMove) {
        if let Some(player) = &mut self.player {
            match player_move {
                PlayerMove::Translate(dx, dy) => player.translate(*dx, *dy),
                PlayerMove::Rotate(angle) => player.rotate(*angle),
            }
        }
    }

    pub fn try_move(&mut self, player_move: PlayerMove) -> Option<Collision> {
        self.do_move(&player_move);
        let collision = self.collision();
        if let Some(_) = collision {
            self.do_move(&player_move.opposite());
        }
        collision
    }

    pub fn stamp(&mut self) {
        if let Some(player) = &self.player {
            for (x, y) in player.extent() {
                let x = x as usize;
                let y = y as usize;
                if y >= ROWS || x >= COLUMNS { continue; }
                self.grid[y][x] = player.color;
            }
        }
    }

    pub fn kill_player(&mut self) {
        self.stamp();
        let cleared_rows = self.find_cleared_rows();
        if cleared_rows.len() > 0 {
            self.give_points(cleared_rows.len());
            self.fill_cleared_rows(cleared_rows);
        }
        self.player = None;
    }

    fn find_cleared_rows(&self) -> Vec<usize> {
        let mut cleared_rows = Vec::new();
        if let Some(player) = &self.player {
            for (_, y) in player.extent() {
                let y = y as usize;
                if y >= ROWS { continue; }
                if !self.grid[y].contains(&0) && !cleared_rows.contains(&y) {
                    cleared_rows.push(y);
                }
            }
        }
        cleared_rows.sort_unstable();
        cleared_rows
    }

    fn fill_cleared_rows(&mut self, cleared_rows: Vec<usize>) {
        let mut jump_length = 1;
        for y in (0..cleared_rows[0]).rev() {
            if cleared_rows.contains(&y) {
                jump_length += 1;
            } else {
                self.grid[y + jump_length] = self.grid[y];
            }
        }

        for  y in 0..jump_length {
            self.grid[y] = [0;COLUMNS];
        }
    }

    pub fn alive(&self) -> bool {
        match self.player {
            Some(_) => true,
            None => false,
        }
    }
}
