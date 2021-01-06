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
        // for each stat
        for mut s in stat.stats.values_mut() {
            let mut new_value = s.value;
            let mut multiplicative_multiplier = 1.0;
            let mut additive_multiplier = 0.0;
            let mut additive = 0.0;
            // find effectors affecting this stat
            for e in effector.effectors.iter() {
                let def = effector_defs
                    .defs
                    .get(&e.effector_key)
                    .expect("Tried to get unknown stat key.");

                // Algo:
                // - Apply all multiplicative multipliers
                // - Apply all additive multipliers
                // - Apply all additives

                // look into the effect of each effector
                for (key, ty) in def.effects.iter() {
                    // if any matches
                    if *key == s.key {
                        // Apply Effector
                        match ty {
                            EffectorType::Additive(v) => additive += v,
                            EffectorType::AdditiveMultiplier(v) => additive_multiplier += v,
                            EffectorType::MultiplicativeMultiplier(v) => {
                                multiplicative_multiplier *= v
                            }
                        }
                    }
                }
            }
            let multiplier = multiplicative_multiplier + additive_multiplier;
            new_value *= multiplier;
            new_value += additive;
            s.value_with_effectors = new_value;
        }
    }
    Ok(())
}
