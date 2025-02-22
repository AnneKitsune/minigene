use minigene::prelude::*;

fn render_system(term: &mut Terminal) -> SystemResult {
    term.clear();
    term.print_string(10, 10, Color::White, Color::Black, "Hello World!");
    term.flush();
    Ok(())
}

fn main() {
    run(vec![render_system.system()]);
}
