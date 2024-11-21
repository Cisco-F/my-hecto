mod terminal;
mod view;
mod buffer;
mod command;

use command::Command;
use crossterm::event::{read, Event};
use view::View;
use std::{io::Error as IoE, panic::{set_hook, take_hook}};
use terminal::Terminal;

pub struct Editor{
    // whether use entered ctrl+q
    quit: bool,
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
        let mut view = View::default();
        let args: Vec<String> = std::env::args().collect();
        if let Some(path) = args.get(1) {
            view.load_file(path);
        }
        Ok(Self {
            quit: false,
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
                Ok(event) => self.evaluate_event(event),
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
    fn evaluate_event(&mut self, event: Event) {
        match Command::try_from(event) {
            Ok(command) => {
                if matches!(command, Command::Quit) {
                    self.quit = true;
                } else {
                    self.view.command_handler(command);
                }
            },
            Err(e) => {
                #[cfg(debug_assertions)]
                panic!("Unsupported command: {e}");
            }
        }
    }
    /// refresh the screen
    /// ignore any errors
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor(self.view.get_position());
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