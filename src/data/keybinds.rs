use crossterm::event::KeyCode;

pub struct Keybind<E> {
    pub key: KeyCode,
    pub event: E,
}

impl<E> Keybind<E> {
    pub fn new(key: KeyCode, event: E) -> Self {
        Keybind { key, event }
    }
}
