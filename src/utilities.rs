pub use rand::prelude::*;
pub use std::time::{Duration, Instant};

pub enum PlayerMove{
    Translate(i32, i32),
    Rotate(i32),
}

impl PlayerMove {
    pub fn opposite(&self) -> PlayerMove {
        match self {
            PlayerMove::Rotate(angle) => PlayerMove::Rotate(-angle),
            PlayerMove::Translate(dx, dy) => PlayerMove::Translate(-dx, -dy),
        }
    }
}

pub enum Collision{
    Wall,
    Floor,
    Block,
}

#[derive(Clone)]
pub struct Shape {
    extent: Vec<(f32, f32)>,
    offset: (f32, f32),
}

impl Shape {
    pub fn parse_shapes(_text: &str) -> Vec<Shape> {
        vec![
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (1.0, -1.0),
                ],
                offset: (1.0, 1.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-1.0, -1.0),
                    (0.0, -1.0),
                    (1.0, -1.0),
                    (-1.0, -0.0),
                    (1.0, -0.0),
                    (-1.0, 1.0),
                    (1.0, 1.0),
                    ],
                offset: (1.0, 1.0),
            },
            Shape {
                extent: vec![
                    (-1.0, -2.0),
                    (0.0, -2.0),
                    (1.0, -2.0),
                    (-1.0, -1.0),
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (1.0, 1.0),
                    (-1.0, 2.0),
                    (0.0, 2.0),
                    (1.0, 2.0),
                    ],
                offset: (1.0, 2.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (-1.0, 1.0),
                    ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (1.0, 1.0),
                    ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (0.0, 1.0),
                    (1.0, 0.0),
                    ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (0.0, 1.0),
                    (1.0, 1.0),
                    ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (0.0, -1.0),
                    (1.0, -1.0),
                    ],
                offset: (1.0, 1.0),
            },
            Shape {
                extent: vec![
                    (-1.0, 0.0),
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (2.0, 0.0),
                    ],
                offset: (1.0, 0.0),
            },
            Shape {
                extent: vec![
                    (-0.5, -0.5),
                    (0.5, -0.5),
                    (-0.5, 0.5),
                    (0.5, 0.5),
                    ],
                offset: (0.5, 0.5),
            },
            Shape {
                extent: vec![
                    (-1.0, -1.0),
                    (0.0, -1.0),
                    (1.0, -1.0),
                    (0.0, 0.0),
                    (0.0, -1.0),
                    ],
                offset: (1.0, 1.0),
            },
            Shape {
                extent: vec![
                    (-1.0, -1.0),
                    (0.0, -1.0),
                    (1.0, -1.0),
                    (0.0, 0.0),
                    (0.0, -1.0),
                    ],
                offset: (1.0, 1.0),
            },
        ]
    }

    pub fn get_offset(&self) -> (f32, f32) {
        (self.offset.0, self.offset.1)
    }

    pub fn rotate(&mut self, angle: i32) {
        for (x, y) in self.extent.iter_mut() {
            let (x2, y2) = match angle.rem_euclid(4) {
                1 => (-*y, *x),
                2 => (-*x, -*y),
                3 => (*y, -*x),
                _ => (*x, *y),
            };
            *x = x2;
            *y = y2;
        }
    }

    pub fn extent(&self) -> Vec<(i32, i32)> {
        self.extent.clone().iter().map(|(x, y)| ((*x + self.offset.0) as i32, (*y + self.offset.1) as i32)).collect()
    }
}

pub struct Player {
    x: i32,
    y: i32,
    shape: Shape,
    pub color: usize,
}

impl Player {
    pub fn spawn(x:i32, y:i32,shape:Shape,color:usize) -> Self {
        Self {
            x,
            y,
            shape,
            color,
        }
    }
    
    pub fn rotate(&mut self, angle: i32) {
        self.shape.rotate(angle);
    }

    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn extent(&self) -> Vec<(i32, i32)> {
        self.shape.extent().into_iter().map(|(x, y)| (x + self.x, y + self.y)).collect()
    }
}

pub struct Fps {
    time: Instant,
    frames: u64,
    pub fps: f64,
    measurement_time: Duration,
}

impl Fps {
    pub fn new(measurement_time: Duration) -> Fps {
        Fps {
            time: Instant::now(),
            frames: 0,
            fps: 0.0,
            measurement_time,
        }
    }

    fn reset(&mut self) {
        self.time = Instant::now();
        self.frames = 0;
    }

    pub fn frame(&mut self) {
        self.frames += 1;
        if self.time.elapsed() > self.measurement_time {
            self.fps = (self.frames as f64)/self.time.elapsed().as_secs_f64();
            self.reset();
        }
    }
}
