/// A raw keyboard event.
#[derive(Debug, Clone, Copy)]
pub enum KeyboardEvent {
    KeyPress { c: char },
}
