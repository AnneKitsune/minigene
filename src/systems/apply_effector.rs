use crate::*;

/// Modifies the stats of entities depending on the effectors applied through them
/// (using the `EffectorSet` component.)
pub fn apply_effector_system<K: Hash + Eq, E: Hash + Eq>(
    effector_defs: &EffectorDefinitions<K, E>,
    effectors: &Components<EffectorSet<E>>,
    stats: &mut Components<StatSet<K>>,
) -> SystemResult {
    for (stat, effector) in join!(&mut stats && &effectors) {
        let stat = stat.unwrap();
        let effector = effector.unwrap();
        effector.apply_to(effector_defs, stat, 0.0);
    }
    Ok(())
}
