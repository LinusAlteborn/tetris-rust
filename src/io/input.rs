use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::event::{KeyEvent, KeyEventKind, KeyCode};

#[derive(Debug)]
pub enum InputEvent {
    Rotate,
    Down,
    Right,
    Left,
    Drop,
    Quit,
}

pub struct Input {
    events: Vec<InputEvent>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            events: Vec::new(),
        }
    }

    pub fn poll(&self) -> Option<InputEvent> {
        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent{kind: KeyEventKind::Press, code, ..}) => match code {
                    KeyCode::Char(' ') | KeyCode::Backspace => Some(InputEvent::Drop),
                    KeyCode::Char('a') | KeyCode::Left => Some(InputEvent::Left),
                    KeyCode::Char('s') | KeyCode::Down => Some(InputEvent::Down),
                    KeyCode::Char('d') | KeyCode::Right => Some(InputEvent::Right),
                    KeyCode::Char('r') | KeyCode::Char('w') | KeyCode::Up => Some(InputEvent::Rotate),
                    KeyCode::Char('q') | KeyCode::Esc => Some(InputEvent::Quit),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    }
}
