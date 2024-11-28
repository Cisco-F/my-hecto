use std::io::Error as IoE;

use super::{line::Line, view::Location};

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
    pub fn insert_at(&mut self, location: Location, c: char) {
        let Location { line_index, .. } = location;
        if line_index >= self.total_lines() {
            self.lines.push(Line::from(&String::from(c)));
        } else {
            self.lines.get_mut(line_index).unwrap().insert_at(location, c);
        }
    }
}

// #[cfg(test)]
// #[test]
// fn test_load_first_line() {
//     let mut buffer = Buffer::default();
//     buffer.load_file("Grapheme.txt").unwrap();
//     let line = buffer.lines.get(0).unwrap();
//     for frag in &line.fragments {
//         let a = &frag.grapheme;
//         let b = &frag.grapheme_width;
//         let c = &frag.replacement.unwrap_or_else(|| 'n');
//         println!("{a:?}, {b:?}, {c:?}");
//     }
// }
