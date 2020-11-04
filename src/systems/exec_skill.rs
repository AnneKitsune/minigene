use crate::*;

pub struct ExecSkillRes<S: Send + Sync + 'static>(pub ReaderId<SkillTriggerEvent<S>>);

system!(
    ExecSkillSystem<
        K: Send + Sync + Hash + Eq + 'static,
        E: Send + Sync + Clone + Hash + Eq + 'static,
        S: Send + Sync + Hash + Eq + 'static,
        I: Send + Sync + 'static
    >, |skill_defs: ReadExpect<'a, SkillDefinitions<K, E, S, I>>,
     skill_instances: WriteStorage<'a, Comp<SkillSet<S>>>,
     stats: ReadStorage<'a, Comp<StatSet<K>>>,
     effector_defs: ReadExpect<'a, EffectorDefinitions<K, E>>,
     effectors: WriteStorage<'a, Comp<EffectorSet<E>>>,
     event_channel: Read<'a, EventChannel<SkillTriggerEvent<S>>>,
     reader: WriteExpect<'a, ExecSkillRes<S>>| {
        for ev in event_channel.read(&mut reader.0) {
            // TODO consume item if needed
            let def = skill_defs
                .defs
                .get(&ev.1)
                .expect("Received event for unknown skill key.");
            for eff in def.stat_effectors.iter() {
                let eff_def = effector_defs.defs.get(&eff).expect("Unknown effector key.");
                effectors
                    .entry(ev.0)
                    .unwrap()
                    .or_insert_with(|| Comp(EffectorSet::default()))
                    .0
                    .effectors
                    .push(EffectorInstance::new(eff.clone(), eff_def.duration));
            }
            skill_instances.get_mut(ev.0)
            .expect("Entity specified by event doesn't have an expected SkillInstance for this skill activation.")
            .0
            .skills
            .get_mut(&ev.1)
            .expect("Skill instance doesn't exist for this entity")
            .current_cooldown = def.cooldown;
        }
    }
);
