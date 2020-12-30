use crate::*;

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
            s.value_with_effectors = s.value;
            // find effectors affecting this stat
            for e in effector.effectors.iter() {
                let def = effector_defs
                    .defs
                    .get(&e.effector_key)
                    .expect("Tried to get unknown stat key.");
                // look into the effect of each effector
                for (key, ty) in def.effects.iter() {
                    // if any matches
                    if *key == s.key {
                        // Apply Effector
                        match ty {
                            EffectorType::Additive(v) => s.value_with_effectors += v,
                            EffectorType::AdditiveMultiplier(_v) => unimplemented!(),
                            EffectorType::MultiplicativeMultiplier(v) => {
                                s.value_with_effectors *= v
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
