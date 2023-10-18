use std::io::{self, stdout, Write};

use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use crate::editor::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    #[allow(clippy::missing_errors_doc)]
    pub fn new() -> Result<Self, io::Error> {
        let (width, height) = terminal_size()?;
        Ok(Self {
            size: Size { width, height },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(pos: &Position) {
        let Position { mut x, mut y } = pos;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", clear::CurrentLine);
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
