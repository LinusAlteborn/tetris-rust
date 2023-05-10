use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use json::object;
use std::fs;
use std::io;
use std::io::Write;

/// Stores all of settings for the session
///
/// The struct stores a difficulty which has the range of 1-9
/// and a color which is a char

#[derive(Clone, Copy)]
pub struct Settings {
    difficulty: u32,
    color: char,
}

/// Stores a user with an assisted score and name
///
/// The struct stores a name as a string and a number as u32
#[derive(Clone)]
pub struct User {
    name: String,
    pub score: u32,
}

impl User {
    /// Creates a new user
    ///
    /// Creates a new user given a name and gives it the starting score 0
    ///  
    /// Arguments:
    ///
    /// name: A string is the users name
    ///
    /// Return:
    ///
    /// A new instance of the struct User
    /// 
    /// Example:
    /// 
    /// add_user(String.from("Carl"))
    /// # => User{name: "Carl", score: 0}
    fn add_user(name: String) -> Self {
        Self { name, score: 0 }
    }
}

/// Stores the top 10 highest scores
///
/// The struct stores a single vec which holds a users
/// The users holds name and a score
#[derive(Clone)]
pub struct Highscores {
    users: Vec<User>,
}

/// Methods for Highscores
impl Highscores {
    /// append adds a new user to the Highscore structs user vec
    ///
    /// This out of place method creates a new Highscore struct with the added user.
    /// If there are more than 10 users in the struct it will compare the new user
    /// to the user which has lowest score and if it has higher it will remove that user.
    /// Then will it sort the vec.
    ///
    /// Arguments:
    ///
    /// self: which is the struct which is calling this method
    /// user: which holds the struct user that stores a score and a name
    ///
    /// Return:
    ///
    /// A new instance of the struct Highscore
    /// 
    /// Example:
    /// 
    /// Highscore{users: [User{"Carl", score: 20}]}.append(User{"Tore": 50})
    /// # => Highscore{users: [User{"Tore", score: 50} ,User{"Carl", score: 20}]}
    pub fn append(&mut self, user: User) -> Self {
        let length = self.users.len();
        if self
            .users
            .iter()
            .min_by_key(|x| x.score)
            .unwrap_or(&User {
                score: 0,
                name: "".to_string(),
            })
            .score
            < user.score
            || length <= 10
        {
            if length == 10 {
                self.users.remove(0);
            }
            self.users.push(user.clone());
            self.users.sort_by_key(|x| x.score);
            self.users.reverse();
        }
        Self {
            users: self.users.clone(),
        }
    }
}

impl Settings {
    pub fn start() -> crossterm::Result<(Settings, User)> {
        let mut result = -1;
        let (high_scores, mut settings) = Self::load_json();
        loop {
            if result == -1 {
                disable_raw_mode().unwrap();
                println!(
                    "Welcome to Tetris\n\nPress 1. to play\nPress 2. for settings\nPress 3. for highscore"
                );
                enable_raw_mode().unwrap();
                result = loop {
                    if let Event::Key(key) = read().unwrap() {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('1') => break 1,
                                KeyCode::Char('2') => break 2,
                                KeyCode::Char('3') => break 3,
                                _ => (),
                            }
                        }
                    }
                }
            }
            disable_raw_mode().unwrap();
            if result == 1 {
                print!("Input user_name: ");
                let mut name = String::new();
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut name)
                    .expect("failed to readline");
                let user = User::add_user(name);
                Self::save_json(settings, high_scores);
                return Ok((settings.clone(), user));
            } else if result == 2 {
                let mut settings_result = -1;
                loop {
                    if settings_result == -1 {
                        println!("Settings\n\nPress 1. for difficulty\nPress 2. for color\nPress 3. for exit");
                        settings_result = 0;
                    } else if settings_result == 0 {
                        enable_raw_mode().unwrap();
                        settings_result = loop {
                            if let Event::Key(key) = read().unwrap() {
                                if key.kind == KeyEventKind::Press {
                                    match key.code {
                                        KeyCode::Char('1') => break 1,
                                        KeyCode::Char('2') => break 2,
                                        KeyCode::Char('3') => break 3,
                                        _ => (),
                                    }
                                }
                            }
                        };
                        if settings_result == 3 {
                            break;
                        }
                        disable_raw_mode().unwrap();
                    }
                    if settings_result == 1 {
                        println!("Set difficulty in the range 1-9");
                        enable_raw_mode().unwrap();
                        let difficulty = loop {
                            if let Event::Key(key) = read().unwrap() {
                                if key.kind == KeyEventKind::Press {
                                    match key.code {
                                        KeyCode::Char(event) => {
                                            if event.is_digit(10) {
                                                break event.to_digit(10).unwrap();
                                            }else{
                                                disable_raw_mode().unwrap();
                                                println!("You need to enter a number in the range 1-9");
                                                enable_raw_mode().unwrap();
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        };
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.difficulty = difficulty;
                    }
                    if settings_result == 2 {
                        println!("Set color\n\nPress r. for red\nPress b. for black");
                        enable_raw_mode().unwrap();
                        let color = loop {
                            if let Event::Key(key) = read().unwrap() {
                                match key.code {
                                    KeyCode::Char('r') => break 'r',
                                    KeyCode::Char('b') => break 'b',
                                    _ => (),
                                }
                            }
                        };
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.color = color
                    }
                }
                result = -1;
                disable_raw_mode().unwrap();
            } else if result == 3 {
                println!("\nHighscore:");
                for x in high_scores.users.iter() {
                    println!("{}: {}", x.name, x.score)
                }
                println!("\nPress any key to exit:");
                enable_raw_mode().unwrap();
                result = loop {
                    if let Event::Key(key) = read().unwrap() {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char(_event) => break -1,
                                _ => (),
                            }
                        }
                    }
                };
                disable_raw_mode().unwrap();
            }
        }
    }

    /// Load the local settings.json file for highscore and settings
    ///
    /// The json file is loaded or if it doesnt exsist an error is raised.
    /// The json is parsed and then placed into a Highscore instant and a instant of settings
    /// Which is then returned
    ///
    /// Return:
    ///
    /// A tuple of an instance of Highscore and an instance of Settings
    /// 
    /// Example:
    /// 
    /// load_json()
    /// # => (Highscore{users: [User{"Tore", score: 50}, User{"Carl", score: 20}]}, Settings{difficulty: 4, color: 'b'})
    pub fn load_json() -> (Highscores, Self) {
        let content = fs::read_to_string("./src/settings.json").expect("error");
        let json = json::parse(&content).unwrap();
        let settings = Settings {
            difficulty: json["difficulty"].as_u32().unwrap(),
            color: json["color"].as_str().unwrap().chars().nth(0).unwrap(),
        };
        let mut high_scores = Highscores { users: vec![] };
        for x in json["highscore"].members() {
            high_scores.users.push(User {
                name: x["name"].to_string(),
                score: x["score"].as_u32().unwrap(),
            })
        }
        return (high_scores, settings);
    }

    /// Save memory to the local settings.json file that stores highscore and settings
    ///
    /// Highscore and settings is parsed as a json and then written into the settings.json file
    /// 
    /// Arguments:
    ///
    /// settings: A instance of Settings which holds the settings
    /// high_scores: A instance of Highscore which holds the highscore
    /// 
    /// Example:
    /// 
    /// save_json(Highscore{users: [User{"Tore", score: 50}, User{"Carl", score: 20}]}, Settings{difficulty: 4, color: 'b'})
    pub fn save_json(settings: Self, high_scores: Highscores) {
        let mut data = object! {
            difficulty: settings.difficulty,
            color: settings.color.to_string()
        };
        data["highscore"] = json::JsonValue::new_array();
        for x in high_scores.users {
            data["highscore"]
                .push(object! {name: x.name, score: x.score})
                .expect("Error occured while reading highscore");
        }
        fs::write("./src/settings.json", data.dump()).expect("Unable to write file");
    }
}
