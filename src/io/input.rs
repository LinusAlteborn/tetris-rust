use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::event::{KeyEvent, KeyEventKind, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug)]
pub enum InputEvent {
    Rotate,
    Down,
    Right,
    Left,
    Drop,
    Quit,
}

pub fn input() -> Option<InputEvent> {
    enable_raw_mode().unwrap();
    let event = if poll(Duration::from_millis(0)).unwrap() {
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
    };
    disable_raw_mode().unwrap();
    event
}
