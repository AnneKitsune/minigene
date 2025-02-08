use crate::Color;
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::Write;

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        let stdout = std::io::stdout();
        crossterm::terminal::enable_raw_mode().unwrap();
        Self { stdout }
    }

    pub fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn print_color(&mut self, x: i32, y: i32, fg: Color, bg: Color, character: char) {
        if x < 0 || y < 0 {
            return;
        }
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .unwrap()
            .queue(style::PrintStyledContent(character.with(fg).on(bg)))
            .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
