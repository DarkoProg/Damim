use crate::Position;

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
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
                height: size.1.saturating_sub(2),
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn clear_current_line() {
        execute!(std::io::stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn set_bg_color(color: Color) {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::style::SetBackgroundColor(color)
        )
        .unwrap();
    }

    pub fn reset_bg_color() {
        crossterm::execute!(std::io::stdout(), ResetColor).unwrap();
    }

    pub fn set_fg_color(color: Color) {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::style::SetBackgroundColor(color)
        )
        .unwrap();
    }

    pub fn reset_fg_color() {
        crossterm::execute!(std::io::stdout(), ResetColor).unwrap();
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        execute!(
            std::io::stdout(),
            MoveTo(position.x as u16, position.y as u16)
        )
        .unwrap();
    }

    pub fn cursor_hide() {
        crossterm::execute!(std::io::stdout(), Hide).unwrap();
    }

    pub fn cursor_show() {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
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
