use crate::*;

/// Moves an entity one tile towards the targeted entity, regardless of collisions.
///
/// # Errors
/// This function returns `Ok(())` under all conditions since it does not generate any errors. It may only panic in case of internal inconsistency.
///
/// # Panics
/// This function panics if:
///   - An entity that has a `GotoEntity` and `Point` component (as found in the first pass) is missing the `Point` component in the second pass (which should be impossible).
///   - The `Entity` id from the first pass is `None` (which should be impossible).
///   - The `GotoEntity` component from the first pass is `None` (which should be impossible).
pub fn goto_entity_simple_system(
    entities: &Entities,
    gotos: &Components<GotoEntity>,
    positions: &mut Components<Point>,
) -> SystemResult {
    let mut v = vec![];
    for (e, _, goto) in join!(&entities && &positions && &gotos) {
        let entity_id = e.expect("Expected an entity id to exist");
        let goto_component = goto.expect("Expected GotoEntity component to exist");
        v.push((entity_id, goto_component.entity, goto_component.speed));
    }
    for (e, t, speed) in v {
        if let Some(target) = positions.get(t).copied() {
            let p = positions
                .get_mut(e)
                .expect("Entity to move must have a position component");
            for _i in 0..(speed as usize) {
                let delta_x = target.x - p.x;
                let delta_y = target.y - p.y;
                match delta_x.cmp(&0) {
                    std::cmp::Ordering::Greater => p.x += 1,
                    std::cmp::Ordering::Less => p.x -= 1,
                    std::cmp::Ordering::Equal => {}
                }
                match delta_y.cmp(&0) {
                    std::cmp::Ordering::Greater => p.y += 1,
                    std::cmp::Ordering::Less => p.y -= 1,
                    std::cmp::Ordering::Equal => {}
                }
            }
        }
    }
    Ok(())
}
