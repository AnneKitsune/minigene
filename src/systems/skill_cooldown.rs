pub use crate::*;

system!(SkillCooldownSystem<S: Send+Sync+Hash+Eq+'static>, |
        skill_instances: WriteStorage<'a, Comp<SkillSet<S>>>,
        time: Read<'a, Time>| {
    for inst in (&mut skill_instances,).join() {
        for i in (inst.0).0.skills.iter_mut() {
            i.1.current_cooldown -= time.delta_seconds() as f64;
        }
    }
});
