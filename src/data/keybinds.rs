use crossterm::event::KeyCode;

/// A mapping from a keycode to an event.
pub struct Keybind<E> {
    /// The keycode
    pub key: KeyCode,
    /// The associated event that gets generated once the keyboard key associated with the keycode is pressed.
    pub event: E,
}

impl<E> Keybind<E> {
    /// Creates a new `Keybind` with the provided keycode and associated event.
    pub const fn new(key: KeyCode, event: E) -> Self {
        Self { key, event }
    }
}
