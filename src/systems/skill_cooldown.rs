use crate::*;

/// Ticks down the cooldown of skill instances for each entity having a `SkillSet` component.
pub fn skill_cooldown_system<S: Hash + Eq>(
    time: &Time,
    skill_instances: &mut Components<SkillSet<S>>,
) -> SystemResult {
    for inst in skill_instances.iter_mut() {
        for i in inst.skills.iter_mut() {
            i.1.current_cooldown -= time.delta_time().as_secs_f64();
        }
    }
    Ok(())
}
