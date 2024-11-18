use crossterm::{
    event::{read, Event::Key, KeyCode::Char}, 
    terminal::{disable_raw_mode, enable_raw_mode}
};

pub struct Editor{}

impl Editor {
    /// default constructor
    pub fn default() -> Self {
        Editor{}
    }
    /// start functioning
    pub fn run(&self) {
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        print!("Thanks for using!");
    }
    /// read-eval-print-loop
    fn repl(&self) -> Result<(), std::io::Error> {
        // throw errors to upper layers
        enable_raw_mode()?;
        loop {
            // read an event, including character event, mouse event, .etc
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}