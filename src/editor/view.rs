use std::cmp::min;

use super::{buffer::Buffer, command::{Command, Direction}, line::Line, terminal::*};

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
    location: Location,
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
            Command::Move(direction) => self.move_location(direction),
            Command::Insert(c) => self.insert(c),
            Command::Resize(size) => self.resize(size),
        }
    }
    /// load file from given path. if file inexists, just panic
    pub fn load_file(&mut self, path: &str) {
        if let Err(e) = self.buffer.load_file(path) {
            panic!("\x1b[31mError when loading file: {e}\x1b[0m");
        }
    }
    /// react to resize event
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.need_redraw = true;
    }

    // region: rendering

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
    /// render a single line at provided row index
    fn render_line(row: usize, text: &str) {
        let ret = Terminal::print_at(row, text);
        debug_assert!(ret.is_ok(), "Failed to render line!");
    }
    fn insert(&mut self, c: char) {
        self.buffer.insert_at(self.location, c);
        self.move_right();
        self.need_redraw = true;
    }

    // region: scrolling

    /// judge if the cursor is out of view's bound
    fn scroll_screen(&mut self) {
        let Position { col, row } = self.loc_to_pos();
        self.scroll_horizontal(col);
        self.scroll_vertical(row);
    }
    fn scroll_horizontal(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let out_of_bound = if to < self.offset.col {
            self.offset.col = to;
            true
        } else if to >= self.offset.col + width {
            self.offset.col = to - width + 1;
            true
        } else {
            false
        };

        self.need_redraw |= out_of_bound;
    }
    fn scroll_vertical(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let out_of_bound = if to < self.offset.row {
            self.offset.row = to;
            true
        } else if to >= self.offset.row + height {
            self.offset.row = to - height + 1;
            true
        } else {
            false
        };

        self.need_redraw |= out_of_bound;
    }

    // region: location & position handling

    /// convert grapheme index to caret position.
    /// due to the introduction of graphemes, some characters might take 2 or more bytes' space, 
    /// but only 1 or 2 space's length shown on screen
    fn loc_to_pos(&self) -> Position {
        let row = self.location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.location.grapheme_index)
        });
        Position { row, col }
    }
    pub fn caret_position(&self) -> Position {
        self.loc_to_pos().subtract(&self.offset)
    }

    // region: text location movement

    /// triggers when user push direction buttons or HOME, END ...
    /// due to the introduction of graphemes, we need to adjust the cursor to navigate among graphemes
    pub fn move_location(&mut self, direction: Direction) {
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height - 1),
            Direction::PageDown => self.move_down(height - 1),
            Direction::Home => self.move_to_line_start(),
            Direction::End => self.move_to_line_end(),
        }
        self.scroll_screen();
    }
    fn move_up(&mut self, step: usize) {
        self.location.line_index = self.location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }
    fn move_down(&mut self, step: usize) {
        self.location.line_index = self.location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }
    fn move_left(&mut self) {
        if self.location.grapheme_index > 0 {
            self.location.grapheme_index -= 1;
        } else {
            self.move_up(1);
            self.move_to_line_end();
        }
    }
    fn move_right(&mut self) {
        let line_len = self
            .buffer
            .lines
            .get(self.location.line_index)
            .map_or(0, Line::grapheme_len);
        if self.location.grapheme_index < line_len {
            self.location.grapheme_index += 1;
        } else {
            self.move_down(1);
            self.move_to_line_start();
        }
    }
    fn move_to_line_end(&mut self) {
        self.location.grapheme_index = self
            .buffer
            .lines
            .get(self.location.line_index)
            .map_or(0, Line::grapheme_len);
    }
    fn move_to_line_start(&mut self) {
        self.location.grapheme_index = 0;
    }
    fn snap_to_valid_grapheme(&mut self) {
        self.location.grapheme_index = self
            .buffer
            .lines
            .get(self.location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_len(), self.location.grapheme_index)
            });
    }
    fn snap_to_valid_line(&mut self) {
        self.location.line_index = min(self.location.line_index, self.buffer.total_lines())
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
}
