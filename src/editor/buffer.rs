use std::io::Error as IoE;

/// buffer that records contents for each line
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    /// generated target contains a String
    #[allow(unused)]
    fn default() -> Buffer {
        Self {
            lines: vec!["Hello World".to_string()]
        }
    }
    /// load file from given path to buffer
    pub fn load_file(&mut self, path: &str) -> Result<(), IoE> {
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
