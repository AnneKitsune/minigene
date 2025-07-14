use crate::Color;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Error as IoError, Write};

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl Terminal {
    // TODO log errors
    pub fn new() -> Result<Self, IoError> {
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok(Self { stdout })
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

    pub fn print_string(&mut self, x: i32, y: i32, fg: Color, bg: Color, string: &str) {
        if x < 0 || y < 0 {
            return;
        }
        let width = terminal::size().unwrap().0 as i32;
        for (i, c) in string.chars().enumerate() {
            if x + i as i32 >= width {
                break;
            }
            self.print_color(x + i as i32, y, fg, bg, c);
        }
    }

    pub fn print_box(&mut self, x: i32, y: i32, width: u32, height: u32, fg: Color, bg: Color) {
        for y_offset in 0..height {
            let current_y = y + y_offset as i32;
            for x_offset in 0..width {
                let current_x = x + x_offset as i32;
                let c: char = if y_offset == 0 {
                    // Top border
                    if x_offset == 0 {
                        '╭'
                    } else if x_offset == width - 1 {
                        '╮'
                    } else {
                        '─'
                    }
                } else if y_offset == height - 1 {
                    // Bottom border
                    if x_offset == 0 {
                        '╰'
                    } else if x_offset == width - 1 {
                        '╯'
                    } else {
                        '─'
                    }
                } else {
                    // Middle rows
                    if x_offset == 0 || x_offset == width - 1 {
                        '│'
                    } else {
                        ' '
                    }
                };
                self.print_color(current_x, current_y, fg, bg, c);
            }
        }
    }

    pub fn get_input(&self) -> Option<KeyCode> {
        // Check if there is any input available without blocking
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key_event) => Some(key_event.code),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn wait_input(&self) -> KeyCode {
        // Check if there is any input available without blocking
        loop {
            match event::read().unwrap() {
                Event::Key(key_event) => return key_event.code,
                _ => {}
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
