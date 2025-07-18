use crate::*;
use uuidmap::Table;

/// Transforms `char` input events into the desired event type using a keybindings map.
///
/// # Errors
/// No errors can be returned.
pub fn input_processor<E: Copy>(
    keymap: &Table<Keybind<E>>,
    inputs: &Table<KeyboardEvent>,
    events: &mut Table<E>,
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
