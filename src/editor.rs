mod terminal;

use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error as IoE;
use terminal::Terminal;

pub struct Editor{
    quit: bool,
}

impl Editor {
    /// default constructor
    pub fn default() -> Self {
        Editor{ quit: false }
    } 
    /// start functioning
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), IoE> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);

            if self.quit == true {
                break;
            }
        }
        Ok(())
    }
    /// evaluate an event, including character event, mouse event, .etc
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent{ 
            code, modifiers, ..
        }) = event {
            println!("[debug]: code: {code:?}, modifiers: {modifiers:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.quit = true;
                },
                _ => (),
            }
        }
    }
}