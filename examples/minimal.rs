use minigene::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
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
