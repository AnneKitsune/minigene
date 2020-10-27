use crate::*;

#[derive(Debug, Clone, new)]
pub struct SkillTriggerEvent<K>(pub Entity, pub K);

system!(TriggerPassiveSkillSystem<K: Send+Sync+Debug+Hash+Eq+'static, E: Send+Sync+'static, S: Send+Sync+Clone+Hash+Eq+'static, I: Send+Sync+'static+Clone+PartialEq+Debug, IT: Send+Sync+'static+SlotType, CD: Send+Sync+'static+Default+Debug+Clone>, |
        skill_defs: ReadExpect<'a, SkillDefinitions<K, E, S, I>>,
        skill_instances: WriteStorage<'a, Comp<SkillSet<S>>>,
        stats: ReadStorage<'a, Comp<StatSet<K>>>,
        stat_defs: ReadExpect<'a, StatDefinitions<K>>,
        inventories: ReadStorage<'a, Comp<Inventory<I, IT, CD>>>,
        event_channel: Write<'a, EventChannel<SkillTriggerEvent<S>>>, 
        entities: Entities<'a>| {
    for (entity, skills, stat, inventory) in (&*entities, &mut skill_instances, &stats, &inventories).join() {
        for skill in skills.0.skills.iter() {
            if skill.1.current_cooldown <= 0.0 {
                // get def from skill key
                let def = skill_defs.defs.get(&skill.0).expect("No skill definition for provided key");
                if def.passive && def.check_conditions(&stat.0, &inventory.0, &stat_defs) {
                    // Trigger skill
                    event_channel.single_write(SkillTriggerEvent(entity, skill.0.clone()));
                }
            }
        }
    }
});
