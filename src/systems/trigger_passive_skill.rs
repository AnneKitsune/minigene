use crate::*;

/// An event where a skill is activated by a specific entity.
#[derive(Debug, Clone, new)]
pub struct SkillTriggerEvent<K>(pub Entity, pub K);

/// Activates passive skills for each entities where all conditions are met using
/// their `SkillSet` component.
/// Creates `SkillTriggerEvent` events.
pub fn trigger_passive_skill_system<
    K: Debug + Hash + Eq,
    E,
    S: Clone + Hash + Eq,
    I: Clone + PartialEq + Debug + Hash + Eq,
    IT: SlotType,
    CD: Default + Debug + Clone + PartialEq,
>(
    skill_defs: &SkillDefinitions<K, E, S, I>,
    stats: &Components<StatSet<K>>,
    stat_defs: &StatDefinitions<K>,
    inventories: &Components<Inventory<I, IT, CD>>,
    entities: &Entities,
    event_channel: &mut Vec<SkillTriggerEvent<S>>,
    skill_instances: &mut Components<SkillSet<S>>,
) -> SystemResult {
    for (entity, skills, stat, inventory) in
        join!(&entities && &mut skill_instances && &stats && &inventories)
    {
        for skill in skills.unwrap().skills.iter() {
            if skill.1.current_cooldown <= 0.0 {
                // get def from skill key
                let def = skill_defs
                    .defs
                    .get(&skill.0)
                    .expect("No skill definition for provided key");
                if def.passive
                    && def.check_conditions(&stat.unwrap(), &inventory.unwrap(), stat_defs)
                {
                    // Trigger skill
                    event_channel.push(SkillTriggerEvent(entity.unwrap(), skill.0.clone()));
                }
            }
        }
    }
    Ok(())
}
