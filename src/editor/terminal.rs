use std::io::{stdout, Error as IoE, Write};
use crossterm::{queue, style::Print, Command};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal();

impl Terminal {
    /// do some initializing job
    pub fn initialize() -> Result<(), IoE> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()?;
        Self::welcome_message()?;
        Self::execute()
    }
    /// do some job before exiting
    pub fn terminate() -> Result<(), IoE> {
        Self::execute()?;
        Self::clear_screen()?;
        Self::reset_cursor()?;
        Self::print("\x1b[32mThanks for using! \r\n")?;
        disable_raw_mode()
    }
    fn welcome_message() -> Result<(), IoE> {
        let str1 = "Welcome!";
        let str2 = "My-hecto    version: 0.1.0";
        let size = Self::size()?;
        let len1 = str1.len() as u16;
        let len2 = str2.len() as u16;
        let start_row = size.height / 3 * 2;
        let start_col1 = (size.width - len1) / 2;
        let start_col2 = (size.width - len2) / 2;
        Self::move_cursor(Position {x: start_col1, y: start_row })?;
        Self::print(str1)?;
        Self::move_cursor(Position { x: start_col2, y: start_row + 1 })?;
        Self::print(str2)?;
        Self::reset_cursor()?;
        Self::execute()
    }
    /// execute all command from command queue
    fn execute() -> Result<(), IoE> {
        stdout().flush()?;
        Ok(())
    }
    /// draw '~' at the start of each line
    fn draw_rows() -> Result<(), IoE> {
        Self::hide_cursor()?;
        Self::reset_cursor()?;

        let y = Self::size().unwrap().height;
        for _ in 0..y {
            Self::print("~\r\n")?;
        }
        Self::reset_cursor()?;
        Self::show_cursor()?;
        Ok(())
    }
    /// clear the screen, send "/x1b[kJ" to the terminal, where k is 1, 2 or NaN
    pub fn clear_screen() -> Result<(), IoE> {
        Self::hide_cursor()?;
        Self::push_command_queue(Clear(ClearType::All))?;
        Self::show_cursor()
    }
    /// size of current terminal
    fn size() -> Result<Size, IoE> {
        let (width, height) = terminal::size()?;
        Ok(Size { width, height })
    }
    /// reset cursor to the top left
    fn reset_cursor() -> Result<(), IoE> {
        Self::move_cursor(Position { x: 0, y: 0 })
    }
    /// move cursor to given position
    fn move_cursor(position: Position) -> Result<(), IoE> {
        Self::push_command_queue(MoveTo(position.x, position.y))
    }
    /// hide terminal cursor. operation will be written in terminal buffer
    fn hide_cursor() -> Result<(), IoE> {
        Self::push_command_queue(Hide)
    }
    /// show terminal cursor. operation will be written in terminal buffer
    fn show_cursor() -> Result<(), IoE> {
        Self::push_command_queue(Show)
    }
    /// use 'Print' with queue! instead of execute!
    fn print(string: &str) -> Result<(), IoE> {
        Self::push_command_queue(Print(string))
    }
    fn push_command_queue<T: Command>(command: T) -> Result<(), IoE> {
        queue!(stdout(), command)?;
        Ok(())
    }
}