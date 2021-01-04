use crate::*;

/// Removes effectors where their time to live is expired.
/// Note: Run after ApplyEffectorsSystem.
pub fn remove_outdated_effector_system<E>(
    time: &Time,
    effectors: &mut Components<EffectorSet<E>>,
) -> SystemResult {
    for eff in effectors.iter_mut() {
        eff.effectors.retain(|e| {
            if let Some(mut d) = e.disable_in {
                d -= time.delta_time().as_secs_f64();
                d > 0.0
            } else {
                true
            }
        });
    }
    Ok(())
}
