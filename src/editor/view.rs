use super::terminal::*;
use std::io::Error as IoE;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

/// contents shown on the screen
pub struct View {
    buffer: Buffer
}

/// buffer that records contents for each line
pub struct Buffer {
    lines: Vec<String>,
}

impl View {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new()
        }
    }
    /// render the terminal window
    pub fn render(&self) -> Result<(), IoE> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Terminal::execute()
    }
    /// draw welcome message; part of initializing work
    fn render_welcome_screen() -> Result<(), IoE> {
        let Size { height, .. } = Terminal::size()?;
        for y in 0..height {
            Terminal::clear_line()?;
            if y == height / 3 * 2 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if y.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    pub fn render_buffer(&self) -> Result<(), IoE> {
        let Size { height, .. } = Terminal::size()?;
        for y in 0..height {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(y as usize) {
                Terminal::print(line)?;
                if y.saturating_add(1) < height {
                    Terminal::print("\r\n")?;
                }
            }
        }
        Ok(())
    }
    /// draw a '~' at the start of the line
    fn draw_empty_row() -> Result<(), IoE> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), IoE> {
        let msg = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size()?.width;
        let padding = (width - msg.len() as u16) / 2;
        let spaces = " ".repeat(padding as usize);
        let msg = format!("~{spaces}{msg}");
        Terminal::print(&msg)?;
        Terminal::execute()
    }
    /// load file from given path. if file inexists, just panic
    pub fn load_file(&mut self, path: &str) {
        if let Err(_) = self.buffer.load_file(path) {
            self.buffer = Buffer::default();
        }
    }
}

impl Buffer {
    fn new() -> Buffer {
        Self {
            lines: vec![]
        }
    }
    /// generated target contains a String
    #[allow(unused)]
    fn default() -> Buffer {
        Self {
            lines: vec!["Hello World".to_string()]
        }
    }
    /// load file from given path to buffer
    fn load_file(&mut self, path: &str) -> Result<(), IoE> {
        let contents = std::fs::read_to_string(path)?;
        for line in contents.lines() {
            self.lines.push(line.to_string());
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}