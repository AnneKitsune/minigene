use crate::InputEvent;
use crossterm::event::KeyCode;

pub struct Keybind {
    pub key: KeyCode,
    pub event: InputEvent,
}
