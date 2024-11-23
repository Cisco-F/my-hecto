use std::io::Error as IoE;

use super::line::Line;

/// buffer that records contents for each line
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

#[allow(dead_code)]
impl Buffer {
    /// load file from given path to buffer
    pub fn load_file(&mut self, path: &str) -> Result<(), IoE> {
        let contents = std::fs::read_to_string(path)?;
        for line in contents.lines() {
            self.lines.push(Line::from(line));
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn total_lines(&self) -> usize {
        self.lines.len()
    }
}
