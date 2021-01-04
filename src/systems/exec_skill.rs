use crate::*;

/// Uses the `SkillTriggerEvent`s to apply effectors to the correct entities.
pub fn exec_skill_system<K: Hash + Eq, E: Clone + Hash + Eq, S: Hash + Eq, I>(
    skill_defs: &SkillDefinitions<K, E, S, I>,
    effector_defs: &EffectorDefinitions<K, E>,
    event_channel: &Vec<SkillTriggerEvent<S>>,
    effectors: &mut Components<EffectorSet<E>>,
    skill_instances: &mut Components<SkillSet<S>>,
) -> SystemResult {
    for ev in event_channel.iter() {
        // TODO consume item if needed
        let def = skill_defs
            .defs
            .get(&ev.1)
            .expect("Received event for unknown skill key.");
        for eff in def.stat_effectors.iter() {
            let eff_def = effector_defs.defs.get(&eff).expect("Unknown effector key.");
            if effectors.get(ev.0).is_none() {
                effectors.insert(ev.0, EffectorSet::default());
            }
            effectors
                .get_mut(ev.0)
                .unwrap()
                .effectors
                .push(EffectorInstance::new(eff.clone(), eff_def.duration));
        }
        skill_instances.get_mut(ev.0)
            .expect("Entity specified by event doesn't have an expected SkillInstance for this skill activation.")
            .skills
            .get_mut(&ev.1)
            .expect("Skill instance doesn't exist for this entity")
            .current_cooldown = def.cooldown;
    }
    Ok(())
}
