//! A minimal example demonstrating the game engine's basic structure
//!
//! This example shows how to:
//! - Set up key bindings
//! - Handle input events
//! - Implement simple rendering
//! - Control the engine's run loop

use minigene::prelude::*;

/// User input events for the minimal example
///
/// These events are generated from keyboard input
/// and processed by engine systems
#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    /// Signals that the engine should stop running
    Quit,
}

fn init_keybinds(keybinds: &mut Table<Keybind<InputEvent>>) -> SystemResult {
    keybinds.add(Keybind::new(KeyCode::Esc, InputEvent::Quit));
    Ok(())
}

fn render_system(term: &mut Terminal) -> SystemResult {
    term.clear();
    term.print_string(10, 10, Color::White, Color::Black, "Hello World!");
    term.flush();
    Ok(())
}

fn quit_system(events: &Table<InputEvent>, running: &mut EngineRunning) -> SystemResult {
    for ev in events.values() {
        match ev {
            InputEvent::Quit => running.running = false,
        }
    }
    Ok(())
}

fn main() {
    run(
        vec![init_keybinds.system()],
        vec![
            render_system.system(),
            input_driver_blocking.system(),
            input_processor::<InputEvent>.system(),
            quit_system.system(),
        ],
    );
}
