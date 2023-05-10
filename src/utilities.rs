pub use rand::prelude::*;
pub use std::time::{Duration, Instant};
use std::vec;

// Denna enum delar upp spelar rörelser i två typer, rotering och flyttning.
pub enum PlayerMove{
    Translate(i32, i32),
    Rotate(i32),
}

impl PlayerMove {
    /// Denna funktion ger tillbaka en PlayerMove med motsatt riktning. (Användbar när jag ska då tillbaka om jag krockat)
    pub fn opposite(&self) -> PlayerMove {
        match self {
            PlayerMove::Rotate(angle) => PlayerMove::Rotate(-angle),
            PlayerMove::Translate(dx, dy) => PlayerMove::Translate(-dx, -dy),
        }
    }
}

/// Denna enum representerar alla typer av kollisioner som kan ske. Det kan vara användbart att skilja dessa åt någon gån i framtiden
pub enum Collision{
    Wall,
    Floor,
    Block,
}

/// Denna struct representerar en form.
/// 
/// Den har en lista av a alla punkter som dess form inehavar och en punk som beskriver ett offset för vart den skal rotera.
#[derive(Clone)]
pub struct Shape {
    extent: Vec<(f32, f32)>,
    offset: (f32, f32),
}

impl Shape {
    /// Denna funktion skulle läsa in en textfil och sedan konvertera denna till en vector av Shape, men det visade sig vara onödigt kompliserat. det är snabbare att hårdkoda in värden istället.
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
                    (0.0, 1.0),
                    ],
                offset: (1.0, 1.0),
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
                    (0.0, 1.0),
                    ],
                offset: (1.0, 1.0),
            },
            Shape {
                extent: vec![
                    (-0.5, -0.5),
                    (0.5, -0.5),
                    (-0.5, 0.5),
                    (0.5, 0.5),
                    (1.5, 0.5)
                    ],
                offset: (0.5, 0.5),
            },
            Shape {
                extent: vec![
                    (-0.5, -0.5),
                    (0.5, -0.5),
                    (-0.5, 0.5),
                    (0.5, 0.5),
                    (1.5, -0.5)
                    ],
                offset: (0.5, 0.5),
            },
        ]
    }

    /// Ännu en så kallad "getter" som låter ossa komma åt ett värde hos struct som igentligen är privat.
    pub fn get_offset(&self) -> (f32, f32) {
        (self.offset.0, self.offset.1)
    }

    /// Denna funktion roterar formen, eftersom vi endast roterar 90 grader i taget är denna kod mycket simpel. Vi byter bara platts på x och y och gör ena negativ. Bam. färdigt.
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

    /// Denna funktion ger tillbaka listan över alla punkter som formen innehavar, denna funktion lägger även på offsetet.
    pub fn extent(&self) -> Vec<(i32, i32)> {
        self.extent.clone().iter().map(|(x, y)| ((*x + self.offset.0) as i32, (*y + self.offset.1) as i32)).collect()
    }
}

/// Denna struct hanterar data relaterat till "spelaren" (det blocket som rör sig)
/// 
/// Den har en position (i32, i32), form (Shape) och färg (usize).
pub struct Player {
    x: i32,
    y: i32,
    shape: Shape,
    pub color: usize,
}

impl Player {
    /// Denna funktion skapar en instans av spelaren. Alla spelarens värden är specificerade i parametrarna (argumenten) och vi använder os av lite syntax sugar för att unvika skriva för mycket.
    /// 
    /// Denna funktion är nögvändig eftersom att vi inte kan skapa en Player struct genom struct literal syntax då fälten är privata. Man kan inte skapa en struct utan att bestäma fältens värden.
    pub fn spawn(x:i32, y:i32,shape:Shape,color:usize) -> Self {
        Self {
            x,
            y,
            shape,
            color,
        }
    }
    
    /// Denna funktion roterar spelare. Den roterar alltså spelarens shape.
    pub fn rotate(&mut self, angle: i32) {
        self.shape.rotate(angle);
    }

    /// Denna funktion förflyttar spelaren. Den muterar helt enkelt x och y värdet hos spelaren.
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Denna funktion ger oss alla positioner som spelarens form innehavar. Denna tar hänsyn till spelarens position och är i ett globalt perspektiv.
    pub fn extent(&self) -> Vec<(i32, i32)> {
        self.shape.extent().into_iter().map(|(x, y)| (x + self.x, y + self.y)).collect()
    }
}

/// Denna struct är till för att simplifiera mätning av fps (Frames Per Second)
/// 
/// Den ser hur många bilder det framkallas under en viss tid och spara detta i dess publika fält fps.
pub struct Fps {
    time: Instant,
    frames: u64,
    pub fps: f64, /// Detta fält
    measurement_time: Duration,
}

impl Fps {
    /// Skapar en ny fps instans med grund värden och specifecerar mättiden i funktionens parameter
    pub fn new(measurement_time: Duration) -> Fps {
        Fps {
            time: Instant::now(),
            frames: 0,
            fps: 0.0,
            measurement_time,
        }
    }

    /// Startar om mätningen för att se hur fps förändras med tid. Ett snitt över hela körtiden på programmet är inte hjälpsamt så fps:n ändras drastigkt under spelets gång.
    fn reset(&mut self) {
        self.time = Instant::now();
        self.frames = 0;
    }

    /// Inkrementerar frame fältet med ett och kollar om tillräckligt lång tid har gått för att utföra beräkning och för att återställa räkningen.
    pub fn frame(&mut self) {
        self.frames += 1;
        if self.time.elapsed() > self.measurement_time {
            self.fps = (self.frames as f64)/self.time.elapsed().as_secs_f64();
            self.reset();
        }
    }
}
