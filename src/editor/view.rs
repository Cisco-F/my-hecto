use super::terminal::*;
use std::io::Error as IoE;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub struct View();

impl View {
    /// render the terminal window
    pub fn render() -> Result<(), IoE> {
        Terminal::clear_line()?;
        Terminal::print("Hello world!\r\n")?;
        let Size { height, .. } = Terminal::size()?;
        for y in 2..height {
            Terminal::clear_line()?;
            if y == height / 3 * 2 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Terminal::execute()
    }
    /// draw welcome message; part of initializing work
    pub fn draw_welcome_message() -> Result<(), IoE> {
        let msg = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size()?.width;
        let padding = (width - msg.len() as u16) / 2;
        let spaces = " ".repeat(padding as usize);
        let msg = format!("~{spaces}{msg}\r\n");
        Terminal::print(&msg)?;
        Terminal::execute()
    }
    /// draw a '~' at the start of the line
    fn draw_empty_row() -> Result<(), IoE> {
        Terminal::print("~\r\n")?;
        Ok(())
    }
}