use crate::*;

pub fn skill_cooldown_system<S: Hash + Eq>(
    skill_instances: &mut Components<SkillSet<S>>,
    time: &Time,
) {
    for inst in skill_instances.iter_mut() {
        for i in inst.skills.iter_mut() {
            i.1.current_cooldown -= time.delta_time().as_secs_f64();
        }
    }
}
