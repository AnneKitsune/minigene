use crate::*;

event_reader_res!(InputDriverRes, VirtualKeyCode);

system!(
    InputDriver<E: Clone + Send + Sync + 'static>,
    |keymap: Read<'a, HashMap<VirtualKeyCode, E>>,
     inputs: Read<'a, EventChannel<VirtualKeyCode>>,
     events: Write<'a, EventChannel<E>>,
     reader: WriteExpect<'a, InputDriverRes>| {
        for i in inputs.read(&mut reader.0) {
            if let Some(e) = keymap.get(i) {
                events.single_write(e.clone());
            }
        }
    }
);
