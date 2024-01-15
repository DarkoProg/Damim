use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use std::fmt;
use std::io::Write;
use std::io::{self};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = size()?;
        enable_raw_mode().unwrap();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn cursor_position(x: u16, y: u16) {
        // let x = x.saturating_add(1);
        // let y = y.saturating_add(1);
        execute!(std::io::stdout(), MoveTo(x, y)).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<KeyEvent, std::io::Error> {
        loop {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                return Ok(key_event);
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}
