use std::io::{stdout, Error as IoE, Write};
use crossterm::{
    style::Print,
    cursor::{Hide, MoveTo, Show},  
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute, queue, 
};

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
        Self::execute()
    }
    /// do some job before exiting
    pub fn terminate() -> Result<(), IoE> {
        stdout().flush()?;
        Self::clear_screen()?;
        Self::reset_cursor()?;
        Self::print("\x1b[32mThanks for using! \r\n")?;
        disable_raw_mode()
    }
    /// clear the screen, send "/x1b[kJ" to the terminal, where k is 1, 2 or NaN
    pub fn clear_screen() -> Result<(), IoE> {
        Self::hide_cursor()?;
        queue!(stdout(), Clear(ClearType::All))?;
        Self::show_cursor()
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
        execute!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    /// hide terminal cursor. operation will be written in terminal buffer
    fn hide_cursor() -> Result<(), IoE> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    /// show terminal cursor. operation will be written in terminal buffer
    fn show_cursor() -> Result<(), IoE> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    /// use 'Print' with queue! instead of execute!
    fn print(string: &str) -> Result<(), IoE> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
}