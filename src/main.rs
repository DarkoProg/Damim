use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{self, stdout, Read};
use std::time::Duration;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    loop {
        if crossterm::event::poll(Duration::from_millis(10))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
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

    disable_raw_mode()?;

    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}
