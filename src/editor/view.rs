use super::{buffer::Buffer, command::{Command, Direction}, terminal::*};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

type Offset = Position;

/// changed position expression to better support graphemes
#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize, 
    pub line_index: usize,
}

/// contents shown on the screen
#[derive(Default)]
pub struct View {
    size: Size,
    buffer: Buffer,
    need_redraw: bool,
    /// current position of the cursor
    position: Position,
    offset: Offset,
}

impl View {
    pub fn default() -> Self {
        Self {
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            ..Default::default()
        }
    }
    pub fn command_handler(&mut self, command: Command) {
        match command {
            Command::Quit => (),
            Command::Move(direction) => self.move_cursor(direction),
            Command::Resize(size) => self.resize(size),
        }
    }
    /// render the terminal window
    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let msg_row = height / 3;
        let offset_row = self.offset.row;
        for row in 0..height {
            if let Some(line) = self.buffer.lines.get(row + offset_row) {
                let left = self.offset.col;
                let right = left + width;
                Self::render_line(row, &line.get_graphems(left..right));
            } else if row == msg_row * 2 && self.buffer.is_empty() {
                Self::render_line(row, &Self::welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }

        self.need_redraw = false;
    }
    /// render a single line
    fn render_line(row: usize, text: &str) {
        let ret = Terminal::print_at(row, text);
        debug_assert!(ret.is_ok(), "Failed to render line!");
    }
    /// triggers when user push direction buttons or HOME, END ...
    pub fn move_cursor(&mut self, direction: Direction) {
        let Size { height, .. } = Terminal::size().unwrap_or_default();
        let Position { col: mut x, row: mut y } = self.position;
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
            Direction::PageUp => y = self.offset.row,
            Direction::PageDown => y = self.offset.row + height - 1,
            Direction::Home => x = 0,
            Direction::End => x = self.buffer.lines.get(y).map_or(0, |line| line.len()),
        }
        x = self.buffer.lines.get(y).map_or(0, |line| x.min(line.len()));
        y = y.min(self.buffer.total_lines());
        self.position = Position{ col: x, row: y };
        self.scroll_screen();
    }
    /// judge if the cursor is out of view's bound
    fn scroll_screen(&mut self) {
        let Position { col: x, row: y } = self.position;
        let Size { width, height } = self.size;
        let mut out_of_bound = false;

        // horizontal
        if x < self.offset.col {
            self.offset.col = x;
            out_of_bound = true;
        } else if x >= self.offset.col + width {
            self.offset.col = x - width + 1;
            out_of_bound = true;
        }

        //vertical
        if y < self.offset.row {
            self.offset.row = y;
            out_of_bound = true;
        } else if y >= self.offset.row + height {
            self.offset.row = y - height + 1;
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
    fn welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let msg = format!("{NAME} -- version {VERSION}");
        let len = msg.len();
        if width <= len {
            return "~".to_string();
        }

        let padding = (width - len) / 2;
        let mut ret = format!("~{}{}", " ".repeat(padding), msg);
        ret.truncate(width);

        ret
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
