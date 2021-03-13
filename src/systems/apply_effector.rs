use crate::*;

/// Modifies the stats of entities depending on the effectors applied through them
/// (using the `EffectorSet` component.)
pub fn apply_effector_system<K: Hash + Eq, E: Hash + Eq>(
    effector_defs: &EffectorDefinitions<K, E>,
    effectors: &Components<EffectorSet<E>>,
    time: &Time,
    stats: &mut Components<StatSet<K>>,
) -> SystemResult {
    for (stat, effector) in join!(&mut stats && &effectors) {
        let stat = stat.unwrap();
        let effector = effector.unwrap();
        effector.apply_to(effector_defs, stat, time.delta_time().as_secs_f32());
    }
    Ok(())
}
