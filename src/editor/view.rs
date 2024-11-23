use super::{buffer::Buffer, command::{Command, Direction}, terminal::*};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

type Offset = Position;

/// contents shown on the screen
#[derive(Default)]
pub struct View {
    pub size: Size,
    buffer: Buffer,
    need_redraw: bool,
    // current position of the cursor
    position: Position,
    pub offset: Offset,
}

impl View {
    pub fn default() -> Self {
        Self {
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            ..Default::default()
        }
    }
    /// render the terminal window
    pub fn render(&mut self) {
        if self.need_redraw {
            if self.buffer.is_empty() {
                Self::render_welcome_screen();
            } else {
                self.render_buffer();
            }
        }
        self.need_redraw = false;
    }
    pub fn command_handler(&mut self, command: Command) {
        match command {
            Command::Quit => (),
            Command::Move(direction) => self.move_cursor(direction),
            Command::Resize(size) => self.resize(size),
        }
    }
    /// draw welcome message; part of initializing work
    fn render_welcome_screen() {
        let Size { height, .. } = Terminal::size().unwrap_or_default();
        for y in 0..height {
            if y == height / 3 * 2 {
                Self::render_line(y, &Self::welcome_message());
            } else {
                Self::render_empty_line(y);
            }
        }
    }
    /// render contents in the buffer, namely the file content
    pub fn render_buffer(&mut self) {
        let Size { height, width } = Terminal::size().unwrap_or_default();
        for y in 0..height {
            let row = self.offset.y;
            if let Some(line) = self.buffer.lines.get(y + row as usize) {
                let left = self.offset.x;
                let right = (left + width).min(line.len() - left);
                // for situation where line's len is bigger than terminal's width, we only render it's child slice
                let truncated_line = &line[left..right];
                Self::render_line(y, truncated_line);
            } else {
                Self::render_empty_line(y);
            }
        }
    }
    /// render a single line
    fn render_line(row: usize, text: &str) {
        let ret = Terminal::print_at(row, text);
        debug_assert!(ret.is_ok(), "Failed to render line!");
    }
    /// draw a '~' at the start of the line
    fn render_empty_line(row: usize) {
        Self::render_line(row, "~");
    }
    /// triggers when user push direction buttons or HOME, END ...
    pub fn move_cursor(&mut self, direction: Direction) {
        let Size { height, .. } = Terminal::size().unwrap_or_default();
        let Position { mut x, mut y } = self.position;
        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = self.buffer.lines.get(y).map_or(0, |line| line.len());
                }
            },
            Direction::Right => {
                let len = self.buffer.lines.get(y).map_or(0, |line| line.len());
                if x < len {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Direction::PageUp => y = self.offset.y,
            Direction::PageDown => y = self.offset.y + height - 1,
            Direction::Home => x = 0,
            Direction::End => x = self.buffer.lines.get(y).map_or(0, |line| line.len()),
        }
        x = self.buffer.lines.get(y).map_or(0, |line| x.min(line.len()));
        y = y.min(self.buffer.total_lines());
        self.position = Position{ x, y };
        self.scroll_screen();
    }
    /// judge if the cursor is out of view's bound
    fn scroll_screen(&mut self) {
        let Position { x, y } = self.position;
        let Size { width, height } = self.size;
        let mut out_of_bound = false;

        // horizontal
        if x < self.offset.x {
            self.offset.x = x;
            out_of_bound = true;
        } else if x >= self.offset.x + width {
            self.offset.x = x - width + 1;
            out_of_bound = true;
        }

        //vertical
        if y < self.offset.y {
            self.offset.y = y;
            out_of_bound = true;
        } else if y >= self.offset.y + height {
            self.offset.y = y - height + 1;
            out_of_bound = true;
        }
        self.need_redraw = out_of_bound;
    }
    /// react to resize event
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.need_redraw = true;
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
        if let Err(e) = self.buffer.load_file(path) {
            panic!("\x1b[31mError when loading file: {e}\x1b[0m");
        }
    }
    /// get cursor position
    /// Attention: corsor position is View.position - View.offset
    pub fn get_position(&self) -> Position {
        self.position.subtract(&self.offset).into()
    }
}
