use crate::*;
use uuidmap::Table;

/// Tries to get a keyboard input event and push it to `Table<KeyboardEvent>`.
///
/// # Errors
/// No errors can occur.
pub fn input_driver(terminal: &Terminal, events: &mut Table<KeyboardEvent>) -> SystemResult {
    if let Some(keycode) = terminal.get_input() {
        events.add(KeyboardEvent::KeyPress { keycode });
    }
    Ok(())
}

/// Gets a keyboard input event and push it to `Table<KeyboardEvent>`.
/// It will wait until such an event is available (a key is pressed)
///
/// # Errors
/// No errors can occur.
pub fn input_driver_blocking(
    terminal: &Terminal,
    events: &mut Table<KeyboardEvent>,
) -> SystemResult {
    let keycode = terminal.wait_input();
    events.add(KeyboardEvent::KeyPress { keycode });
    Ok(())
}
