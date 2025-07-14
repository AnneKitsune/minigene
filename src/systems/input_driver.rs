use crate::*;
use uuidmap::Table;

pub fn input_driver(terminal: &Terminal, events: &mut Table<KeyboardEvent>) -> SystemResult {
    if let Some(keycode) = terminal.get_input() {
        events.add(KeyboardEvent::KeyPress { keycode });
    }
    Ok(())
}
