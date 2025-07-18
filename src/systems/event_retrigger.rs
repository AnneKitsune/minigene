use crate::*;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Sends input events to output events using a map to link the input to the
/// output in O(1).
/// Does not consume input events.
///
/// # Errors
/// Currently, this function always returns `Ok(())`. In the future, we should handle
/// the case of missing inputs appropriately.
pub fn event_retrigger_system<I: Hash + Eq, O: Clone, S: BuildHasher>(
    inputs: &[I],
    mapping: &HashMap<I, O, S>,
    outputs: &mut Vec<O>,
) -> SystemResult {
    let mut all_found = true;
    for i in inputs {
        if let Some(o) = mapping.get(i) {
            outputs.push(o.clone());
        } else {
            all_found = false;
        }
    }
    if !all_found {
        // TODO handle error
    }
    Ok(())
}
