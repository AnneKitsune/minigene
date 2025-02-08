use crossterm::{QueueableCommand, terminal, cursor, style::{self, Stylize}};

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        let mut stdout = std::io::stdout();
        Self {stdout}
    }

    fn clear(mut self) {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    fn flush(mut self) {
        self.stdout.flush().unwrap();
    }
}
