use crate::*;

/// Moves an entity one tile towards the targeted entity, regardless of collisions.
pub fn goto_entity_simple_system(
    entities: &Entities,
    gotos: &Components<GotoEntity>,
    positions: &mut Components<Point>,
) -> SystemResult {
    let mut v = vec![];
    for (e, _, goto) in join!(&entities && &positions && &gotos) {
        v.push((e.unwrap(), goto.unwrap().entity, goto.unwrap().speed));
    }
    for (e, t, speed) in v {
        if let Some(target) = positions.get(t).copied() {
            let p = positions.get_mut(e).unwrap();
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
