use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::error::Error;
use std::fmt;
use std::io::{self, stdout, Read};
use std::time::Duration;

#[derive(Debug)]
struct CustomError(String);

// Implement the Error trait for the custom error type
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom Error: {}", self.0)
    }
}
impl Error for CustomError {}

// Function to handle errors
fn die(err: impl Error) {
    panic!("{}", err);
    // Additional error handling logic can be added here
}

fn main() -> io::Result<()> {
    enable_raw_mode().unwrap();

    loop {
        if crossterm::event::poll(Duration::from_millis(10))? {
            if let crossterm::event::Event::Key(key_event) =
                crossterm::event::read().map_err(die).unwrap()
            {
                match key_event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    } => {
                        break;
                    }
                    _ => {
                        println!("{:?}", key_event.code);
                    }
                }
            }
        }
    }

    disable_raw_mode().unwrap();

    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}
