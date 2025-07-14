use crossterm::event::KeyCode;

/// A raw keyboard event.
#[derive(Debug, Clone, Copy)]
pub enum KeyboardEvent {
    KeyPress { keycode: KeyCode },
}
