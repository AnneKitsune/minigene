use crossterm::event::KeyCode;

/// A raw keyboard event.
#[derive(Debug, Clone, Copy)]
pub enum KeyboardEvent {
    /// Represents a key press event
    KeyPress {
        /// The key that was pressed
        keycode: KeyCode,
    },
}
