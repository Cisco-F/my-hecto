mod terminal;

use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, *}, KeyEvent, KeyModifiers};
use std::{cmp::min, io::Error as IoE};
use terminal::{Position, Size, Terminal};

pub struct Editor{
    // whether use entered ctrl+q
    quit: bool,
    // current position of the cursor
    position: Position,
}

impl Editor {
    /// default constructor
    pub fn default() -> Self {
        Editor{ 
            quit: false,
            position: Position::default(),
        }
    } 
    /// start functioning
    pub fn run(&mut self) {
        if Terminal::size().unwrap().height < 2 {
            panic!("Too small terminal!");
        }
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), IoE> {
        loop {
            let event = read()?;
            // update after user enters something
            self.evaluate_event(&event)?;
            self.refresh_screen()?;

            if self.quit {
                break;
            }
        }
        Ok(())
    }
    /// evaluate an event and distribute to corresponding method
    fn evaluate_event(&mut self, event: &Event) -> Result<(), IoE> {
        if let Key(KeyEvent{ 
            code, 
            modifiers, 
            kind: _, 
            state: _,
        }) = event {
            // println!("[debug]: code: {code:?}, modifiers: {modifiers:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.quit = true;
                },
                Up | Down | Left | Right | PageUp | PageDown | Home | End => {
                    self.move_cursor(*code)?;
                },
                _ => (),
            }
        }
        Ok(())
    }
    fn move_cursor(&mut self, code: KeyCode) -> Result<(), IoE> {
        let Size { width, height } = Terminal::size()?;
        let Position { mut x, mut y } = self.position;
        match code {
            Up => {
                y = y.saturating_sub(1);
            },
            Down => {
                y = min(y + 1, height);
            },
            Left => {
                x = x.saturating_sub(1);
            },
            Right => {
                x = min(x + 1, width);
            }
            PageUp => {
                y = 0;
            },
            PageDown => {
                y = height;
            },
            Home => {
                x = 0;
            },
            End => {
                x = width;
            }
            _ => (),
        }
        self.position = Position{ x, y };
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), IoE> {
        Terminal::hide_cursor()?;
        self.draw_rows()?;
        Terminal::move_cursor(Position {
            x: self.position.x,
            y: self.position.y,
        })?;
        Terminal::show_cursor()?;
        Terminal::execute()
    }
    /// draw '~' at the start of each line
    pub fn draw_rows(&self) -> Result<(), IoE> {
        Terminal::reset_cursor()?;
        Terminal::clear_screen()?;
        let y = Terminal::size().unwrap().height;
        for _ in 0..y {
            Terminal::print("~\r\n")?;
        }
        // Self::reset_cursor()?;
        Terminal::execute()
    }
}