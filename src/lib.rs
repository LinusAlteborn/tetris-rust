pub mod io;
pub use io::{input::*, output::*};

mod menu;
pub use menu::*;

mod utilities;
pub use utilities::*;

/// Set how tall the game screen is
pub const ROWS: usize = 20;
/// Set how wide the game screen is
pub const COLUMNS: usize = 16;

/// This struct stores all data about the current state of the game.
/// 
/// It also has usefull functions for manipulating the data and interacting with it.
/// 
/// fields:
/// grid: [[usize;COLUMNS];ROWS] - a nested array with the gamaeboard grid
/// player: Option<Player> - the moving block
/// shapees: Vec<Shape> - A vec of all possible shapes
/// shape_order: Vec<usize> - Saying which order the shapes should come
/// points: usize - the amount of point accumelated
pub struct GameState {
    grid: [[usize;COLUMNS];ROWS],
    player: Option<Player>,
    shapes: Vec<Shape>,
    shape_order: Vec<usize>,
    pub points: usize,
}

impl GameState {
    /// Create a new GameState with base values
    /// 
    /// Return:
    /// GameState{grid: [[usize;COLUMNS];ROWS], player: Option<Player>, shapes: Vec<Shape>, shape_order: Vec<usize>, point: usize} - The intlized state of the game
    /// 
    /// Example:
    /// 
    /// new()
    /// #=> GameState{grid: [[], [], [], [], [], [], [], []], player: None, shapes: Shape {extent: vec![
    /// (-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (1.0, -1.0),],offset: (1.0, 1.0),}, shape_order: vec![3,2,2,1,0],
    /// point: 0}
    pub fn new() -> Self {
        let shapes = Shape::parse_shapes("");
        Self {
            grid: [[0;COLUMNS];ROWS],
            player: None,
            shapes,
            shape_order: vec![3,2,2,1,0],
            points: 0,
        }
    }

    /// This method decides the points given when clearing rows
    /// 
    /// Argument:
    /// self: GameState - An instance of Gamestate
    /// rows_cleared: usize - How many rows which has been cleared.
    /// 
    /// Exemple:
    /// 
    /// GameState{grid: [[], [], [], [], [], [], [], []], player: None, shapes: Shape {extent: vec![
    /// (-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (1.0, -1.0),],offset: (1.0, 1.0),}, shape_order: vec![3,2,2,1,0],
    /// point: 0}.give_points(2)
    /// #=> GameState{grid: [[], [], [], [], [], [], [], []], player: None, shapes: Shape {extent: vec![
    /// (-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (1.0, -1.0),],offset: (1.0, 1.0),}, shape_order: vec![3,2,2,1,0],
    /// point: 400}
    fn give_points(&mut self, rows_cleared: usize) {
        self.points += 100 * 2usize.pow(rows_cleared as u32);
    }

    /// This function fills the shape_order vector whenever it has been emptied
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// 
    fn fill_shape_order(&mut self) {
        let mut rng = rand::thread_rng();
        let mut nums: Vec<usize> = (0..self.shapes.len().clone()).collect();
        nums.shuffle(&mut rng);
        self.shape_order = nums;
    }

    /// This method "consumes" one number in the shape_order vector.
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// 
    /// Return:
    /// 
    /// usize - the index of th next shape to use
    fn next_shape_index(&mut self) -> usize {
        match self.shape_order.pop() {
            Some(shape) => shape,
            None => {
                self.fill_shape_order();
                self.next_shape_index()
            },
        }
    }

    /// This method spawnes a new player by gathering the next shapes index and then gathering the shape based on the index
    /// Then getting the x and y cordinates of the shape and then the color
    /// Then spawning the character
    /// 
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// 
    /// 
    pub fn spawn(&mut self) {
        let shape_index = self.next_shape_index();
        let shape = self.shapes[shape_index].clone();
        let (x, y) = ((COLUMNS as f32 / 2.0 - shape.get_offset().0) as i32, (0.0) as i32);
        let color = thread_rng().gen_range(1..COLORS.len());
        self.player = Some(Player::spawn(x, y, shape, color));
    }

    /// This method checks for collisions, When it finds one.
    /// It returns a value from an Enum with what type of collission, if non were found it returns None
    /// 
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// 
    /// Return:
    /// 
    /// Option<Collision> - Either a None or an enum value from Collision
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

    /// This function manipulates the player field to move it around
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// player_move: &PlayerMove - An enum value of which direction to move
    /// 
    /// 
    fn do_move(&mut self, player_move: &PlayerMove) {
        if let Some(player) = &mut self.player {
            match player_move {
                PlayerMove::Translate(dx, dy) => player.translate(*dx, *dy),
                PlayerMove::Rotate(angle) => player.rotate(*angle),
            }
        }
    }

    /// This method tries to move or rotate the player. If it collides it goes back to it's original position and returns information about the collision.
    /// 
    /// Argument: 
    /// self: GameState - An instance of Gamestate 
    /// player_move: PlayerMove - An enum value of which direction to move
    /// 
    /// Return:
    /// Option<Collision> - Either None or a Collision
    pub fn try_move(&mut self, player_move: PlayerMove) -> Option<Collision> {
        self.do_move(&player_move);
        let collision = self.collision();
        if let Some(_) = collision {
            self.do_move(&player_move.opposite());
        }
        collision
    }

    /// This function Goes through all the positions the player extends to and sets the grid[y][x] at these positions to the players color.
    fn stamp(&mut self) {
        if let Some(player) = &self.player {
            for (x, y) in player.extent() {
                let x = x as usize;
                let y = y as usize;
                if y >= ROWS || x >= COLUMNS { continue; }
                self.grid[y][x] = player.color;
            }
        }
    }

    /// This function stamps all the players blocks onto the grid. It also checks for cleared rows and gives points for these. The player field is left at None indicating the the player is gone.
    pub fn kill_player(&mut self) {
        self.stamp();
        let cleared_rows = self.find_cleared_rows();
        if cleared_rows.len() > 0 {
            self.give_points(cleared_rows.len());
            self.fill_cleared_rows(cleared_rows);
        }
        self.player = None;
    }

    /// This function finds all rows that have been cleared. It only checks the rows which the player is occupying to save processing time.
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

    /// This is a nice, optimized algorithm for moving down the rows after some have been cleared.
    /// 
    /// It iterates over all rows and displaces them by the jump_height variable. the jump_height variable increments by one every time we iterate on a cleared row.
    fn fill_cleared_rows(&mut self, cleared_rows: Vec<usize>) {
        let mut jump_length = 1;
        if cleared_rows.len() == 0 { return; }
        for y in (0..*cleared_rows.last().unwrap()).rev() {
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

    /// This is something called a getter. It's a way for users of the GameStruct to access the structs private fields
    pub fn alive(&self) -> bool {
        match self.player {
            Some(_) => true,
            None => false,
        }
    }
}
