use std::ops::{Add, Sub};
pub use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

use crate::COLUMNS;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn from_pos(x: f32, y: f32) -> Point {
        Point { x, y, }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(Clone)]
pub struct Shape {
    extent: Vec<Point>,
    center: Point,
}

impl Shape {
    pub fn shapes_from(text: &str) -> Vec<Shape> {
        vec![
            Shape {
                extent: vec![
                    Point::from_pos(-1.0, 0.0),
                    Point::from_pos(0.0, 0.0),
                    Point::from_pos(1.0, 0.0),
                    Point::from_pos(-1.0, 1.0),
                ],
                center: Point::from_pos(1.0, 0.0),
            },
            Shape {
                extent: vec![
                    Point::from_pos(-1.0, 0.0),
                    Point::from_pos(0.0, 0.0),
                    Point::from_pos(1.0, 1.0),
                    Point::from_pos(1.0, 0.0),
                ],
                center: Point::from_pos(1.0, 0.0),
            },
            Shape {
                extent: vec![
                    Point::from_pos(-0.5, -0.5),
                    Point::from_pos(0.5, -0.5),
                    Point::from_pos(0.5, 0.5),
                    Point::from_pos(-0.5, 0.5),
                ],
                center: Point::from_pos(0.5, 0.5),
            },
            Shape {
                extent: vec![
                    Point::from_pos(-1.0, 0.0),
                    Point::from_pos(0.0, 0.0),
                    Point::from_pos(1.0, 0.0),
                    Point::from_pos(2.0, 0.0),
                ],
                center: Point::from_pos(1.0, 0.0),
            },
        ]
    }

    pub fn get_spawn_pos(&self) -> Point {
        Point::from_pos((COLUMNS  / 2 - self.center.x as usize) as f32 - 1.0, 0.0)
    }

    pub fn rotated(&self, delta: i32) -> Shape {
        let mut extent: Vec<Point> = Vec::with_capacity(self.extent.len());
        for point in self.extent.iter() {
            if delta > 0 {
                extent.push(Point::from_pos(-point.y, point.x));
            }
            if delta < 0 {
                extent.push(Point::from_pos(point.y, -point.x));
            }
        }
        Shape {
            extent,
            center: self.center,
        }
    }

    pub fn to_world_pos(&self, pos: Point) -> Vec<Point> {
        let mut extent = Vec::with_capacity(self.extent.len());
        for point in self.extent.iter() {
            extent.push(pos + *point + self.center);
        }
        extent
    }
}

pub struct Fps {
    time: Instant,
    frames: u64,
    measurement_time: Duration,
}

impl Fps {
    pub fn new(measurement_time: Duration) -> Fps {
        Fps {
            measurement_time,
            time: Instant::now(),
            frames: 0,
        }
    }

    fn reset(&mut self) {
        self.time = Instant::now();
        self.frames = 0;
    }

    pub fn frame(&mut self) {
        self.frames += 1;
        if self.time.elapsed() >= self.measurement_time {
            println!("fps: {}", (self.frames as u128 * 1000)/(self.time.elapsed().as_millis()));
            self.reset();
        }
    }
}
