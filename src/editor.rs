mod terminal;
mod view;

use crossterm::event::{read, Event::{self, Key, Resize}, KeyCode::{self, *}, KeyEvent, KeyModifiers};
use view::View;
use std::{cmp::min, io::Error as IoE, panic::{set_hook, take_hook}};
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
    pub fn init() -> Result<Self, IoE> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        if Terminal::size().unwrap().height < 2 {
            panic!("Too small terminal!");
        }

        Terminal::initialize()?;
        let mut view = View::new();
        let args: Vec<String> = std::env::args().collect();
        if let Some(path) = args.get(1) {
            view.load_file(path);
        }
        Ok(Self {
            quit: false,
            position: Position::default(),
            view,
        })
    }
    /// start functioning
    pub fn run(&mut self) {
        self.repl();
    }
    /// read-eval-print-loop
    fn repl(&mut self) {
        loop {
            self.refresh_screen();
            if self.quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event, error: {e}");
                    }
                }
            }
        }
    }
    /// evaluate an event and distribute to corresponding method
    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Key(KeyEvent { 
                code, 
                modifiers, 
                kind: _, 
                state: _,
            }) => match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.quit = true;
                },
                Up | Down | Left | Right | PageUp | PageDown | Home | End => {
                    self.move_cursor(*code);
                },

                _ => (),
            },
            Resize(_, _) => {
                self.view.need_render();
            },
            _ => (),
        }
    }
    /// triggers when user push direction buttons or HOME, END ...
    fn move_cursor(&mut self, code: KeyCode) {
        let Size { width, height } = Terminal::size().unwrap_or_default();
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
    }
    /// refresh the screen
    /// ignore any errors
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor(Position {
            x: self.position.x,
            y: self.position.y,
        });
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.quit {
            let _ = Terminal::print("\x1b[32mThanks for using! \r\n");
        }
    }
}