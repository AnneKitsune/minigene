use crate::*;

// Run after ApplyEffectorsSystem
pub fn RemoveOutdatedEffectorSystem<E: Send + Sync + 'static>
    (effectors: &mut Components<EffectorSet<E>>, time: Time) {
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
    }
