use crate::*;
use uuidmap::Table;

// TODO convert to tables

/// Transforms `char` input events into the desired event type using a keybindings map.
pub fn input_driver(
    keymap: &Table<Keybind>,
    inputs: &Table<KeyboardEvent>,
    events: &mut Table<InputEvent>,
) -> SystemResult {
    for i in inputs.values() {
        let KeyboardEvent::KeyPress { c: key } = i;
        for kb in keymap.values() {
            if kb.key == *key {
                events.add(kb.event);
            }
        }
    }
    Ok(())
}
