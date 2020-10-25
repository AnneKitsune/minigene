system!(ApplyEffectorSystem<K: Send+Sync+Hash+Eq+'static, E: Send+Sync+Hash+Eq+'static>, |
        defs: ReadExpect<'a, StatDefinitions<K>>,
        stats: WriteStorage<'a, Comp<StatSet<K>>>,
        effector_defs: ReadExpect<'a, EffectorDefinitions<K,E>>,
        effectors: ReadStorage<'a, Comp<EffectorSet<E>>>| {
    for (stat, effector) in (&mut stats, &effectors).join() {
        // for each stat
        for mut s in stat.0.stats.values_mut() {
            s.value_with_effectors = s.value;
            // find effectors affecting this stat
            for e in effector.0.effectors.iter() {
                let def = effector_defs.defs.get(&e.effector_key).expect("Tried to fetch unknown stat key.");
                // look into the effect of each effector
                for (key, ty) in def.effects.iter() {
                    // if any matches
                    if *key == s.key {
                        // Apply Effector
                        match ty {
                            EffectorType::Additive(v) => s.value_with_effectors += v,
                            EffectorType::AdditiveMultiplier(v) => unimplemented!(),
                            EffectorType::MultiplicativeMultiplier(v) => s.value_with_effectors *= v,
                        }
                    }
                }
            }
        }
    }
});
