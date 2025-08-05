use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Sends input events to output events using a map to link the input to the
/// output in O(1).
/// Does not consume input events.
pub fn event_retrigger_system<I: Hash + Eq, O: Clone, S: BuildHasher>(
    inputs: &[I],
    mapping: &HashMap<I, O, S>,
    outputs: &mut Vec<O>,
) {
    for i in inputs {
        if let Some(o) = mapping.get(i) {
            outputs.push(o.clone());
        }
    }
}
