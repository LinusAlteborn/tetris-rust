pub use rand::prelude::*;
pub use std::time::{Duration, Instant};

/// Denna enum delar upp spelar rörelser i två typer, rotering och flyttning.
pub enum PlayerMove{
    Translate(i32, i32),
    Rotate(i32),
}

impl PlayerMove {
    /// Denna funktion ger tillbaka en PlayerMove med motsatt riktning. (Användbar när jag ska flytta tillbaka om spelaren krockat)
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
    /// Beskrivning: Denna funktion är till för att läsa in en textfil och sedan konvertera denna till en vector av Shape, men det visade sig vara onödigt kompliserat. det är snabbare att hårdkoda in värden istället.
    /// 
    /// Argument 1: &str - Detta är texten från en fil som definerar formerna
    /// 
    /// Return: Vec<Shape> - En lista av alla former (Shapes) som definerades i text filen
    /// 
    /// Exempel: 
    /// parse_shapes("###\n#   \n(1.0, 0.0)") -> [Shape { extent: [(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (-1.0, 1.0)], offset: (1.0, 0.0) }]
    /// parse_shapes("##\n##\n(0.5, 0.5)") -> [Shape { extent: [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)], offset: (0.5, 0.5) }]
    pub fn parse_shapes(_text: &str) -> Vec<Shape> {
        // Warning. Denna funktion är inte implementerad som den ska. Just nu läser den inte filen, utan alla former är hårdkodade.
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
    /// 
    /// Argument 1: &self - Shape structen
    /// 
    /// Return: (f32, f32) - formens offset.
    /// 
    /// Exempel:
    ///     shape.get_offset() -> (0.5, 0.5)
    ///     shape.get_offset() -> (1.0, 0.0)
    pub fn get_offset(&self) -> (f32, f32) {
        (self.offset.0, self.offset.1)
    }

    /// Denna funktion roterar formen, eftersom vi endast roterar 90 grader i taget är denna kod mycket simpel. Vi byter bara platts på x och y och gör ena negativ. Bam. färdigt.
    /// 
    /// Argument 1(self): &mut self - en muterbar reference till formen
    /// Argument 2(angle): i32 - hur många 90 graders rotationer som skal göras
    /// 
    /// Exempel:
    ///     self.rotate(1);
    ///     self.rotate(3);
    ///     self.rotate(-5);
    ///     self.rotate(69);
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
    /// 
    /// Argument 1(self): &self - en icke muterbar reference till en Shape instans
    /// 
    /// Return: Vec<(i32, i32)> - listan över alla punkter som former innehavar med inräknat offset.
    /// 
    /// Exempel:
    ///     self.extent() -> [(0, 0), (1, 0), (2, 0), (0, 1)]
    ///     self.extent() -> [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]
    ///     self.extent() -> [(0, 0), (1, 0), (0, 1), (1, 1)]
    pub fn extent(&self) -> Vec<(i32, i32)> {
        self.extent.clone().iter().map(|(x, y)| ((*x + self.offset.0) as i32, (*y + self.offset.1) as i32)).collect()
    }
}

/// Denna struct hanterar data relaterat till "spelaren" (det blocket som rör sig)
/// 
/// Spelarens fält är:
///     x: spelarens x position
///     y: spelarens y position
///     shape: spelarens nuvarande form.
///     coloe: färd indexet som spelaren har
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
    /// 
    /// Argument 1(x): i32 - spelarens x position
    /// Argument 2(y): i32 - spelarens y position
    /// Argument 3(shape): Shape - spelarens nuvarande funktion
    /// Argument 4(color): usize - indexet för färgen som spelaren har
    /// 
    /// Return: Player - en ny instans av en spelare
    /// 
    /// Exempel:
    ///     Player::spawn(1, 2, Shape { extent: [(0.0, 0.0),], offset: (0.0, 0.0) }, 69) -> Player { x: 1, y: 2, shape: Shape { extent: [(0.0, 0.0),], offset: (0.0, 0.0) }, color: 69}
    pub fn spawn(x:i32, y:i32,shape:Shape,color:usize) -> Self {
        Self {
            x,
            y,
            shape,
            color,
        }
    }
    
    /// Denna funktion roterar spelare. Den roterar alltså spelarens shape.
    /// 
    /// Argument 1(self): &mut self - en muterbar reference till spelaren
    /// Argument 2(angle): i32 - mängden 90 graders roteringar vi vill göra
    /// 
    /// Exempel:
    ///     self.rotate(69);
    ///     self.rotate(-1);
    ///     self.rotate(1);
    ///     self.rotate(420);
    pub fn rotate(&mut self, angle: i32) {
        self.shape.rotate(angle);
    }

    /// Denna funktion förflyttar spelaren. Den muterar helt enkelt x och y värdet hos spelaren.
    /// 
    /// Argument 1(self): &mut self - en muterbar reference till spelaren
    /// Argument 2(delta_x): i32 - mängden förflyttning i x led
    /// Argument 3(delta_y): i32 - mängden förflyttning i y led
    /// 
    /// Exempel:
    ///     self.translate(0, 0);
    ///     self.translate(0, 1);
    ///     self.translate(-5, 5);
    ///     self.translate(-69, 420);
    pub fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Denna funktion ger oss alla positioner som spelarens form innehavar. Denna tar hänsyn till spelarens position och är i ett globalt perspektiv.
    /// 
    /// Argument 1(self): Player - en icke muterbar reference till spelaren
    /// 
    /// Return: Vec<(i32, i32)> - en vector över alla punkter som spelaren innehavar
    /// 
    /// Exempel:
    ///     self.extent() -> [(0, 0), (1, 0), (2, 0), (0, 1)]
    ///     self.extent() -> [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]
    ///     self.extent() -> [(0, 0), (1, 0), (0, 1), (1, 1)]
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
