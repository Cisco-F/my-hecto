use crossterm::{
    cursor::MoveTo, event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers}, execute, terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType}
};
use std::io::{stdout, Error as IoE};

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
    fn initialize() -> Result<(), IoE> {
        enable_raw_mode()?;
        Self::draw_rows()?;
        Self::clear_screen()
    }
    /// do some job before exiting
    fn terminate() -> Result<(), IoE> {
        disable_raw_mode()
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), IoE> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);

            if self.quit == true {
                Self::clear_screen()?;
                print!("Thanks for using! \r\n");
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
    fn clear_screen() -> Result<(), IoE> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn move_cursor(x: u16, y: u16) -> Result<(), IoE> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    fn draw_rows() -> Result<(), IoE> {
        let (_x, y) = terminal::size()?;
        println!("[debug] terminal size, x: {_x}, y: {y}");
        for i in 0..=y {
            print!("~\r\n");
            // Self::move_cursor(0, y+1)?;
        }
        Self::move_cursor(0, 0)?;
        Ok(())
    }
}