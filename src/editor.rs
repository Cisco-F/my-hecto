use crossterm::{
    event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers}, 
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}, 
    execute,
};
use std::io::stdout;

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
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    /// do some initializing job
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    /// do some job before exiting
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);

            if self.quit == true {
                Self::clear_screen()?;
                print!("Thanks for using");
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
    /// clear the screen, send "/x1b[kJ" to the terminal, where k is 1, 2 or NaN
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
}