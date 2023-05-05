use tetris::{*, io::input::InputEvent};
use std::time::{Duration};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{ Write};
use std::fs;
use json::{object};
use std::io;

#[derive(Clone, Copy)]
struct Settings {
    difficulty: u32,
    color: char,
}

#[derive(Clone)]
struct User {
    name: String,
    score: u32
}


impl User{
    fn add_user(name : String) -> Self{
        Self{
            name: name,
            score:  0
        }
    }
}

/// Stores the top 10 highest scores
/// 
/// The struct stores a single vec which holds a users
/// The users holds name and a score
#[derive(Clone)]
struct Highscores {
    users: Vec<User>
}

impl Highscores{
    fn append(&mut self, user: User) -> Self{
        let length = self.users.len();
        if self.users.iter().min_by_key(|x| x.score).unwrap_or(&User{score:0, name: "".to_string()}).score < user.score || length <= 10 {
            if length == 10{
                self.users.remove(0);
            }
            self.users.push(user.clone());
            self.users.sort_by_key( |x| x.score);
            self.users.reverse();
        }
        Self { users: self.users.clone() }
    }
}

impl Settings {
    fn start() -> crossterm::Result<(Settings, User)> {
        let mut result = -1;
        let (high_scores, mut settings) = Self::load_json();
        loop {
            if result == -1{
                disable_raw_mode().unwrap();
                println!(
                    "Welcome to Tetris\n\nPress 1. to play\nPress 2. for settings\nPress 3. for highscore"
                );
                enable_raw_mode().unwrap();
                if let Event::Key(key) = read().unwrap() {
                    match key.code {
                        KeyCode::Char('1') => result = 1,
                        KeyCode::Char('2') => result = 2,
                        KeyCode::Char('3') => result = 3,
                        _ => (),
                    }
                }
            }
            disable_raw_mode().unwrap();
            if result == 1 {
                print!("Input user_name: ");
                let mut name = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name).expect("failed to readline");
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
                        if let Event::Key(key) = read().unwrap() {
                            match key.code {
                                KeyCode::Char('1') => settings_result = 1,
                                KeyCode::Char('2') => settings_result = 2,
                                KeyCode::Char('3') => break,
                                _ => (),
                            }
                        }
                        disable_raw_mode().unwrap();
                    }
                    if settings_result == 1 {
                        println!("Set difficulty in the range 1-9");
                        enable_raw_mode().unwrap();
                        let mut difficulty: Option<char> = None;
                        if let Event::Key(key) = read().unwrap() {
                            match key.code {
                                KeyCode::Char(event) => {
                                    if event.is_digit(10) {
                                        difficulty = Some(event);
                                    }
                                }
                                _ => (),
                            }
                        }
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.difficulty = difficulty.unwrap().to_digit(10).unwrap();
                    }
                    if settings_result == 2 {
                        println!("Set color");
                        enable_raw_mode().unwrap();
                        let mut color: Option<char> = None; 
                        if let Event::Key(key) = read().unwrap() {
                            match key.code {
                                KeyCode::Char('r') => color = Some('r'),
                                KeyCode::Char('b') => color = Some('b'),
                                _ => (),
                            }
                        }
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.color = color.unwrap()
                    }
                }
                result = -1;
                disable_raw_mode().unwrap();
            } else if result == 3 {
                println!("\nHighscore:");
                for x in high_scores.users.iter(){
                    println!("{}: {}", x.name, x.score)
                }
                println!("\nPress any key to exit:");
                enable_raw_mode().unwrap();
                if let Event::Key(key) = read().unwrap() {
                    match key.code {
                        KeyCode::Char(_event) => result = -1,
                        _ => (),
                    }
                }
                disable_raw_mode().unwrap();
            }

        
        }
    }

    fn load_json() -> (Highscores, Self){
        let content = fs::read_to_string("./src/settings.json").expect("error");
        let json = json::parse(&content).unwrap();
        let settings = Settings {
                difficulty: json["difficulty"].as_u32().unwrap(),
                color: json["color"].as_str().unwrap().chars().nth(0).unwrap(),
            };
        let mut high_scores = Highscores {
            users: vec![]
        };
        for x in json["highscore"].members(){
            high_scores.users.push(User {name: x["name"].to_string(), score: x["score"].as_u32().unwrap()})

        }
        return (high_scores, settings)
    }

    fn save_json(settings : Self, high_scores: Highscores) {
        let mut data = object!{
            difficulty: settings.difficulty,
            color: settings.color.to_string()
        };
        data["highscore"] = json::JsonValue::new_array();
        for x in high_scores.users{
            data["highscore"].push(object!{name: x.name, score: x.score}).expect("Error occured while reading highscore");
        }
        fs::write("./src/settings.json", data.dump()).expect("Unable to write file");
    }
}

fn main() {
    Settings::start();
    let mut system = System::new();
    let mut fps = Fps::new(Duration::from_millis(1000));
    loop {
        match system.input.poll() {
            Some(event) => match event {
                InputEvent::Left => system.try_move(Move::Translate(Point::from_pos(-1.0, 0.0))),
                InputEvent::Right => system.try_move(Move::Translate(Point::from_pos(1.0, 0.0))),
                InputEvent::Down => system.try_move(Move::Translate(Point::from_pos(0.0, 1.0))),
                InputEvent::Rotate => system.try_move(Move::Rotate(1)),
                InputEvent::Drop => system.try_move(Move::Drop),
                InputEvent::Quit => {
                    println!("Buh, bye!");
                    std::process::exit(0);
                }
            },
            None => (),
        }

        system.check_move_timer();

        for loop_event in system.events.drain(..) {
            match loop_event {
                Gameloop_Events::Death => {
                    println!("You lose");
                    std::process::exit(0);
                }
                _ => (),
            }
        }

        system.output.update(&system.data);

        fps.frame();
    }
}
