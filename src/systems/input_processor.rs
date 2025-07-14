use crate::*;
use uuidmap::Table;

/// Transforms `char` input events into the desired event type using a keybindings map.
pub fn input_processor(
    keymap: &Table<Keybind>,
    inputs: &Table<KeyboardEvent>,
    events: &mut Table<InputEvent>,
) -> SystemResult {
    for i in inputs.values() {
        let KeyboardEvent::KeyPress { keycode } = i;
        for kb in keymap.values() {
            if kb.key == *keycode {
                events.add(kb.event);
            }
        }
    }
    Ok(())
}
