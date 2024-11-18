use crossterm::{
    event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers}, 
    terminal::{disable_raw_mode, enable_raw_mode}
};

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
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        print!("Thanks for using!");
    }
    /// read-eval-print-loop
    fn repl(&mut self) -> Result<(), std::io::Error> {
        // throw errors to upper layers
        enable_raw_mode()?;
        loop {
            // read an event, including character event, mouse event, .etc
            if let Key(KeyEvent{ 
                code, modifiers, kind, state
            }) = read()? {
                println!("code: {code:?}, modifiers: {modifiers:?}, kind: {kind:?}, state: {state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.quit = true;
                    },
                    _ => (),
                }
            }
            if self.quit == true {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}