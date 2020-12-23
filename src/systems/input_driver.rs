use crate::*;

pub fn InputDriver<E: Clone + Send + Sync + 'static>(
    keymap: &HashMap<VirtualKeyCode, E>,
     inputs: &Vec<VirtualKeyCode>,
     events: &mut Vec<E>) {
        for i in inputs.iter() {
            if let Some(e) = keymap.get(i) {
                events.push(e.clone());
            }
        }
}
