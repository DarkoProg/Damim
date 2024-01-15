use crate::Terminal;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::fmt;
use std::io::Error;
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to init terminal!"),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Sadge!");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => self.should_quit = true,
            _ => println!("{:?}", pressed_key.code),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom Error: {}", self.0)
    }
}

fn die(err: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", err);
}
