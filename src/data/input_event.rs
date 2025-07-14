use crate::Direction;

/// A player input event, converted from a key press.
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Move { direction: Direction },
}
