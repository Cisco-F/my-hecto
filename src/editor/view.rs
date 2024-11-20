use super::terminal::*;
use std::io::Error as IoE;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

/// contents shown on the screen
pub struct View {
    buffer: Buffer,
    need_redraw: bool,
}

/// buffer that records contents for each line
pub struct Buffer {
    lines: Vec<String>,
}

impl View {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            need_redraw: true,
        }
    }
    /// render the terminal window
    pub fn render(&mut self) -> Result<(), IoE> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else if self.need_redraw {
            self.render_buffer()?;
        }
        Terminal::execute()
    }
    /// render a single line
    fn render_line(row: usize, text: &str) -> Result<(), IoE> {
        Terminal::move_cursor(Position { x: 0, y: row })?;
        Terminal::clear_line()?;
        Terminal::print(text)?;
        Ok(())
    }
    /// draw a '~' at the start of the line
    fn render_empty_line(row: usize) -> Result<(), IoE> {
        Self::render_line(row, "~")?;
        Ok(())
    }
    /// draw welcome message; part of initializing work
    fn render_welcome_screen() -> Result<(), IoE> {
        let Size { height, .. } = Terminal::size()?;
        for y in 0..height {
            if y == height / 3 * 2 {
                Self::render_line(y, &Self::welcome_message())?;
            } else {
                Self::render_empty_line(y)?;
            }

            if y.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    /// render contents in the buffer, namely the file content
    pub fn render_buffer(&mut self) -> Result<(), IoE> {
        let Size { height, width } = Terminal::size()?;
        for y in 0..height {
            if let Some(line) = self.buffer.lines.get(y as usize) {
                // for situation where line's len is bigger than terminal's width, we only render it's child slice
                let truncated_line = if line.len() >= width as usize {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(y, truncated_line)?;
            } else {
                Self::render_empty_line(y)?;
            }
        }

        self.need_redraw = false;
        Ok(())
    }
    /// returns a string including project name and version
    fn welcome_message() -> String {
        let msg = format!("{NAME} -- version {VERSION}");
        let width = Terminal::size().unwrap().width;
        let padding = (width - msg.len()) / 2;
        let spaces = " ".repeat(padding);
        format!("~{spaces}{msg}")
    }
    /// load file from given path. if file inexists, just panic
    pub fn load_file(&mut self, path: &str) {
        if let Err(_) = self.buffer.load_file(path) {
            self.buffer = Buffer::default();
        }
    }
    pub fn need_render(&mut self) {
        self.need_redraw = true;
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