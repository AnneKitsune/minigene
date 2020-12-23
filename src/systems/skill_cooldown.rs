use crate::*;

pub fn 
    SkillCooldownSystem<S: Send + Sync + Hash + Eq + 'static>
    (skill_instances: &mut Components<SkillSet<S>>, time: &Time) {
        for inst in skill_instances.iter_mut() {
            for i in inst.skills.iter_mut() {
                i.1.current_cooldown -= time.delta_time().as_secs_f64();
            }
        }
    }
