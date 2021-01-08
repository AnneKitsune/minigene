use crate::*;

/// Transforms `char` input events into the desired event type using a keybindings map.
pub fn input_driver<E: Clone>(
    keymap: &HashMap<char, E>,
    inputs: &Vec<char>,
    events: &mut Vec<E>,
) -> SystemResult {
    for i in inputs.iter() {
        if let Some(e) = keymap.get(i) {
            events.push(e.clone());
        }
    }
    Ok(())
}
