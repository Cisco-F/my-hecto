use std::io::{stdout, Error as IoE, Write};
use crossterm::{queue, style::Print, Command};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn default() -> Position {
        Position{ x: 0, y: 0 }
    }
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal();

impl Terminal {
    /// do some initializing work
    pub fn initialize() -> Result<(), IoE> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()
    }
    /// do some work before exiting
    pub fn terminate() -> Result<(), IoE> {
        Self::execute()?;
        Self::clear_screen()?;
        Self::reset_cursor()?;
        Self::print("\x1b[32mThanks for using! \r\n")?;
        disable_raw_mode()
    }
    /// execute all command from command queue
    pub fn execute() -> Result<(), IoE> {
        stdout().flush()?;
        Ok(())
    }
    /// clear the screen, send "/x1b[kJ" to the terminal, where k is 1, 2 or NaN
    pub fn clear_screen() -> Result<(), IoE> {
        Self::push_command_queue(Clear(ClearType::All))
    }
    /// clear current line
    pub fn clear_line() -> Result<(), IoE> {
        Self::push_command_queue(Clear(ClearType::CurrentLine))
    }
    /// size of current terminal
    pub fn size() -> Result<Size, IoE> {
        let (width, height) = terminal::size()?;
        Ok(Size { width, height })
    }
    /// reset cursor to the top left
    pub fn reset_cursor() -> Result<(), IoE> {
        Self::move_cursor(Position { x: 0, y: 0 })
    }
    /// move cursor to given position
    pub fn move_cursor(position: Position) -> Result<(), IoE> {
        Self::push_command_queue(MoveTo(position.x, position.y))
    }
    /// hide terminal cursor. operation will be written in terminal buffer
    pub fn hide_cursor() -> Result<(), IoE> {
        Self::push_command_queue(Hide)
    }
    /// show terminal cursor. operation will be written in terminal buffer
    pub fn show_cursor() -> Result<(), IoE> {
        Self::push_command_queue(Show)
    }
    /// use 'Print' with queue! instead of execute!
    pub fn print(string: &str) -> Result<(), IoE> {
        Self::push_command_queue(Print(string))
    }
    /// push a command to the command queue
    fn push_command_queue<T: Command>(command: T) -> Result<(), IoE> {
        queue!(stdout(), command)?;
        Ok(())
    }
    // pub fn exit_failure(error: IoE) {
    //     Self::clear_screen().unwrap();
    //     Self::reset_cursor().unwrap();
    //     Self::print("my-hecto panic because of error: {error}!").unwrap();
    //     disable_raw_mode().unwrap();
    //     exit(1);
    // }
}