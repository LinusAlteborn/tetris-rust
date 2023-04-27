use tetris::{*, io::input::InputEvent};
use std::time::{Duration};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, terminal, ExecutableCommand, Result};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::io::{stdout, Write};
use std::fs;



struct Settings {
    difficulty: u32,
    color: char,
}

struct User {
    name: String,
    score: u32
}

struct Highscores {
    users: Vec<User>
}

impl Settings {
    fn start() -> crossterm::Result<Settings> {
        let content = fs::read_to_string("./src/settings.json").expect("error");
        let json = json::parse(&content).unwrap();
        let mut result = -1;
        let mut settings = Settings {
                difficulty: json["Difficulty"].as_u32().unwrap(),
                color: json["color"].as_str().unwrap().chars().nth(0).unwrap(),
            };
        let mut high_scores = Highscores {
            users: vec![]
        };
        for x in json["HighScore"].members(){
            high_scores.users.push(User {name: x["name"].to_string(), score: x["score"].as_u32().unwrap()})

        }
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
                return Ok(settings);
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
                        let difficulty: Option<char> = loop {
                            if let Event::Key(key) = read().unwrap() {
                                match key.code {
                                    KeyCode::Char(event) => {
                                        if event.is_digit(10) {
                                            break Some(event);
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        };
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.difficulty = difficulty.unwrap().to_digit(10).unwrap();
                    }
                    if settings_result == 2 {
                        println!("Set color");
                        enable_raw_mode().unwrap();
                        let color: Option<char> = loop {
                            if let Event::Key(key) = read().unwrap() {
                                match key.code {
                                    KeyCode::Char('r') => break Some('r'),
                                    KeyCode::Char('b') => break Some('b'),
                                    _ => (),
                                }
                            }
                        };
                        settings_result = -1;
                        disable_raw_mode().unwrap();
                        settings.color = color.unwrap()
                    }
                }
                result = -1;
                disable_raw_mode().unwrap();
            }
        
        }
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
