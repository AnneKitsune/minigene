use crate::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Sends input events to output events using a map to link the input to the
/// output in O(1).
/// Does not consume input events.
pub fn event_retrigger_system<I: Hash + Eq, O: Clone>(
    inputs: &[I],
    mapping: &HashMap<I, O>,
    outputs: &mut Vec<O>,
) -> SystemResult {
    let mut all_found = true;
    for i in inputs.iter() {
        if let Some(o) = mapping.get(i) {
            outputs.push(o.clone());
        } else {
            all_found = false;
        }
    }
    if all_found {
        Ok(())
    } else {
        // TODO error
        Ok(())
    }
}
