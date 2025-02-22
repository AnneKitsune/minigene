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

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn print_string(&mut self, x: i32, y: i32, fg: Color, bg: Color, string: String) {
        string.chars().enumerate().for_each(|(i, c)| {
            self.print_color(x + i as i32, y, fg, bg, c);
        });
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
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
