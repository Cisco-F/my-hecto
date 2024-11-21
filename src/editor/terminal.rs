use std::io::{stdout, Error as IoE, Write};
use crossterm::{queue, style::Print, Command};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn default() -> Position {
        Position{ x: 0, y: 0 }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Terminal();

#[allow(unused)]
impl Terminal {
    /// do some initializing work
    pub fn initialize() -> Result<(), IoE> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()
    }
    /// do some work before exiting
    pub fn terminate() -> Result<(), IoE> {
        Self::leave_alternate_screen()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()
    }
    pub fn enter_alternate_screen() -> Result<(), IoE> {
        Self::push_command_queue(EnterAlternateScreen)
    }
    pub fn leave_alternate_screen() -> Result<(), IoE> {
        Self::push_command_queue(LeaveAlternateScreen)
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
        let width = width as usize;
        let height = height as usize;
        Ok(Size { width, height })
    }
    /// reset cursor to the top left
    pub fn reset_cursor() -> Result<(), IoE> {
        Self::move_cursor(Position { x: 0, y: 0 })
    }
    /// move cursor to given position
    pub fn move_cursor(position: Position) -> Result<(), IoE> {
        Self::push_command_queue(MoveTo(position.x as u16, position.y as u16))
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
    pub fn print(text: &str) -> Result<(), IoE> {
        Self::push_command_queue(Print(text))
    }
    pub fn print_at(row: usize, text: &str) -> Result<(), IoE> {
        Self::move_cursor(Position { x: 0, y: row })?;
        Self::clear_line()?;
        Self::print(text)
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