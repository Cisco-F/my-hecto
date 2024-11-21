use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::terminal::Size;

pub enum Direction {
    Up,
    Down, 
    Left, 
    Right, 
    PageUp, 
    PageDown, 
    Home, 
    End,
}

pub enum Command {
    Move(Direction), 
    Resize(Size),
    Quit,
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => Ok(Self::Quit),
                    KeyCode::Up => Ok(Self::Move(Direction::Up)),
                    KeyCode::Down => Ok(Self::Move(Direction::Down)),
                    KeyCode::Left => Ok(Self::Move(Direction::Left)),
                    KeyCode::Right => Ok(Self::Move(Direction::Right)),
                    KeyCode::PageUp => Ok(Self::Move(Direction::PageUp)),
                    KeyCode::PageDown => Ok(Self::Move(Direction::PageDown)),
                    KeyCode::Home => Ok(Self::Move(Direction::Home)),
                    KeyCode::End => Ok(Self::Move(Direction::End)),
                    _ => Err("Unsupported key: {code:?}".to_string()),
                }
            },
            Event::Resize(width_u16, height_u16) => {
                let width = width_u16 as usize;
                let height = height_u16 as usize;
                Ok(Self::Resize(Size { width, height }))
            },
            _ => Err("Unsupported event: {event:?}".to_string()),
        }
    }
}