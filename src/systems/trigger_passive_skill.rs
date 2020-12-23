use crate::*;

#[derive(Debug, Clone, new)]
pub struct SkillTriggerEvent<K>(pub Entity, pub K);

pub fn trigger_passive_skill_system<
    K: Debug + Hash + Eq,
    E,
    S: Clone + Hash + Eq,
    I: Clone + PartialEq + Debug,
    IT: SlotType,
    CD: Default + Debug + Clone,
> (
    skill_defs: &Option<SkillDefinitions<K, E, S, I>>,
    stats: &Components<StatSet<K>>,
    stat_defs: &Option<StatDefinitions<K>>,
    inventories: &Components<Inventory<I, IT, CD>>,
    event_channel: &mut Vec<SkillTriggerEvent<S>>,
    entities: &Entities,
    skill_instances: &mut Components<SkillSet<S>>,
) -> SystemResult {
    for (entity, skills, stat, inventory) in
        join!(&entities && &mut skill_instances && &stats && &inventories)
    {
        for skill in skills.unwrap().skills.iter() {
            if skill.1.current_cooldown <= 0.0 {
                // get def from skill key
                let def = skill_defs
                    .as_ref()
                    .unwrap()
                    .defs
                    .get(&skill.0)
                    .expect("No skill definition for provided key");
                if def.passive
                    && def.check_conditions(
                        &stat.unwrap(),
                        &inventory.unwrap(),
                        stat_defs.as_ref().unwrap(),
                    )
                {
                    // Trigger skill
                    event_channel.push(SkillTriggerEvent(entity.unwrap(), skill.0.clone()));
                }
            }
        }
    }
    Ok(())
}
