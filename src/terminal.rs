use crate::Color;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize as _},
    terminal, ExecutableCommand as _, QueueableCommand as _,
};
use std::io::{stdout, Error as IoError, Write as _};

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().expect("Failed to create Terminal with raw mode enabled.")
    }
}

impl Terminal {
    // TODO log errors

    /// Creates a new `Terminal`.
    /// It will have raw mode enabled by default.
    ///
    /// # Errors
    /// Will return an error if it fails to enable raw mode for the terminal.
    pub fn new() -> Result<Self, IoError> {
        let stdout = stdout();
        terminal::enable_raw_mode()?;
        Ok(Self { stdout })
    }

    /// Clears the screen so that it is blank.
    ///
    /// # Panics
    /// This function may panic if it fails to send `terminal::Clear` to the terminal.
    pub fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Failed to send terminal::Clear to the terminal");
    }

    /// Flushes stdout's queued content to the terminal.
    ///
    /// # Panics
    /// Will panic if failing to flush stdout.
    pub fn flush(&mut self) {
        self.stdout.flush().expect("Failed to flush terminal");
    }

    /// Prints a single colored character to the terminal.
    ///
    /// # Panics
    /// Will panic if failing to queue terminal events to stdout.
    pub fn print_color(&mut self, x: i32, y: i32, fg: Color, bg: Color, character: char) {
        if x < 0 || y < 0 {
            return;
        }
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Failed to queue cursor::MoveTo")
            .queue(style::PrintStyledContent(character.with(fg).on(bg)))
            .expect("Failed to queue style::PrintStyledContent");
    }

    /// Prints a colored string to the terminal.
    ///
    /// # Panics
    /// Will panic if it fails to detect the terminal size.
    pub fn print_string(&mut self, x: i32, y: i32, fg: Color, bg: Color, string: &str) {
        if x < 0 || y < 0 {
            return;
        }
        // TODO handle the error by assuming the terminal is 1x1 and logging an info message.
        let width = terminal::size().expect("Failed to get terminal size").0 as i32;
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

    /// Attempts to get a queud keyboard input event from the terminal.
    /// Returns `None` if no event is available.
    ///
    /// # Panics
    /// May panic if the attached terminal is closed.
    pub fn get_input(&self) -> Option<KeyCode> {
        // Check if there is any input available without blocking
        if event::poll(std::time::Duration::from_millis(0)).expect("Failed to poll for terminal events") {
            if let Event::Key(key_event) = event::read().expect("Failed to read terminal event.") {
                Some(key_event.code)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the next terminal keyboard input event.
    /// Blocks until the next event.
    ///
    /// # Panics
    /// May panic if the attached terminal is closed.
    pub fn wait_input(&self) -> KeyCode {
        // Check if there is any input available without blocking
        loop {
            if let Event::Key(key_event) = event::read().expect("Failed to read terminal event.") {
                return key_event.code;
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
