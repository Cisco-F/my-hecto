mod terminal;
mod view;

use crossterm::event::{read, Event::{self, Key}, KeyCode::{self, *}, KeyEvent, KeyModifiers};
use view::View;
use std::{cmp::min, io::Error as IoE};
use terminal::{Position, Size, Terminal};

pub struct Editor{
    // whether use entered ctrl+q
    quit: bool,
    // current position of the cursor
    position: Position,
    /// contents shown on the screen
    view: View,
}

impl Editor {
    /// default constructor
    pub fn default() -> Self {
        Editor{ 
            quit: false,
            position: Position::default(),
            view: View::new(),
        }
    } 
    /// start functioning
    pub fn run(&mut self) {
        if Terminal::size().unwrap().height < 2 {
            panic!("Too small terminal!");
        }
        Terminal::initialize().unwrap();
        self.parse_param();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), IoE> {
        loop {
            self.refresh_screen()?;
            if self.quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
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
        Terminal::reset_cursor()?;
        self.view.render()?;
        Terminal::move_cursor(Position {
            x: self.position.x,
            y: self.position.y,
        })?;
        Terminal::show_cursor()?;
        Terminal::execute()
    }
    pub fn parse_param(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        if let Some(path) = args.get(1) {
            self.view.load_file(path);
        }
    }
}