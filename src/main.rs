use crossterm::{
    execute, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::error::Error;
use std::io::{self, stdout, Read};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    loop {
        if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
            if key_event.code == crossterm::event::KeyCode::Char('q') {
                if key_event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;

    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}
