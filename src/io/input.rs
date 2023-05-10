use std::time::Duration;

use crossterm::event::{poll, read, Event};
use crossterm::event::{KeyEvent, KeyEventKind, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

/// En abstraction av alla olika input som jag kan få
/// 
/// Varje typ är en agering som jag kan ta i spelet. Jag kan rotera, flytta mig i många olika håll, jag kan snabbfalla och jag kan avsluta spelet
pub enum InputEvent {
    Rotate,
    Down,
    Right,
    Left,
    Drop,
    Quit,
}

/// Denna funktionen hanterar interaktionen mellan tangenttryck och event i spelet. Den konverterar alltså knapptryck till en rörelse/händelse i spelet.
/// 
/// Return: Option<InputEvent> - antingen en None, eller en Some(InputEvent) som beskriver ett spelares input
/// 
/// Exempel:
///     input() -> None;
///     input() -> Some(InputEvent::Drop);
///     input() -> Some(InputEvent::Left);
///     input() -> Some(InputEvent::Right);
///     input() -> Some(InputEvent::Down);
///     input() -> Some(InputEvent::Rotate);
pub fn input() -> Option<InputEvent> {
    enable_raw_mode().unwrap();
    let event = if poll(Duration::ZERO).unwrap() {
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
