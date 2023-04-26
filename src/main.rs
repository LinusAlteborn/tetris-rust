use tetris::{*, io::input::InputEvent};
use std::time::{Duration};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};



struct Settings {
    difficulty: u32,
    color: char,
}

impl Settings {
    fn start() -> crossterm::Result<Settings> {
        println!(
            "Welcome to Tetris\n\nPress 1. to play\nPress 2. for settings\nPress 3. for highscore"
        );

        enable_raw_mode().unwrap();
        let result = loop {
            if let Event::Key(key) = read().unwrap() {
                match key.code {
                    KeyCode::Char('1') => break 1,
                    KeyCode::Char('2') => break 2,
                    KeyCode::Char('3') => break 3,
                    _ => (),
                }
            }
        };
        disable_raw_mode().unwrap();
        let mut settings = Settings {
            difficulty: 1,
            color: 'b',
        };
        if result == 1 {
            return Ok(settings);
        } else if result == 2 {
            let mut result = -1;
            loop {
                if result == -1 {
                    println!("Settings\n\nPress 1. for difficulty\nPress 2. for color\nPress 3. for exit");
                    result = 0;
                } else if result == 0 {
                    enable_raw_mode().unwrap();
                    if let Event::Key(key) = read().unwrap() {
                        match key.code {
                            KeyCode::Char('1') => result = 1,
                            KeyCode::Char('2') => result = 2,
                            KeyCode::Char('3') => break,
                            _ => (),
                        }
                    }
                    disable_raw_mode().unwrap();
                }
                if result == 1 {
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
                    result = -1;
                    disable_raw_mode().unwrap();
                    settings.difficulty = difficulty.unwrap().to_digit(10).unwrap();
                }
                if result == 2 {
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
                    result = -1;
                    disable_raw_mode().unwrap();
                    settings.color = color.unwrap()
                }
            }
            disable_raw_mode().unwrap();
        }
        Ok(settings)
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
