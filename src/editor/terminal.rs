use std::io::{stdout, Error as IoE};
use crossterm::{
    cursor::MoveTo, execute, 
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType}
};

pub struct Terminal();

impl Terminal {
    /// do some initializing job
    pub fn initialize() -> Result<(), IoE> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()
    }
    /// do some job before exiting
    pub fn terminate() -> Result<(), IoE> {
        Self::clear_screen()?;
        Self::reset_cursor()?;
        print!("Thanks for using! \r\n");
        disable_raw_mode()
    }
    /// clear the screen, send "/x1b[kJ" to the terminal, where k is 1, 2 or NaN
    pub fn clear_screen() -> Result<(), IoE> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    /// draw '~' at the start of each line
    fn draw_rows() -> Result<(), IoE> {
        Self::move_cursor(0,0)?;
        let y = terminal::size()?.1;

        for _ in 0..y {
            print!("~\r\n");
        }
        Self::reset_cursor()?;
        Ok(())
    }
    /// reset cursor to the top left
    pub fn reset_cursor() -> Result<(), IoE> {
        Self::move_cursor(0, 0)
    }
    /// move cursor to given position
    pub fn move_cursor(x: u16, y: u16) -> Result<(), IoE> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
}