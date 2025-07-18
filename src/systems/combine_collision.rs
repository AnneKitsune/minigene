use crate::*;

/// Combine individual entity's `CollisionMap` components into one single
/// `CollisionResource` resource.
///
/// # Panics
/// Panics if:
/// - `global_map` is `None`
/// - Any entity in the join for `positions` and `collisions` has a missing component
/// - Any entity in the join for `positions` and `maps` has a missing component
///
/// # Errors
/// This function does not return any error and only returns `Ok(())`.
pub fn combine_collision_system(
    positions: &Components<Point>,
    collisions: &Components<Collision>,
    maps: &Components<CollisionMap>,
    global_map: &mut Option<CollisionResource>,
) -> SystemResult {
    let global_map = global_map.as_mut().expect("global_map must be set");

    global_map.map.clear();

    for (pos, _) in join!(&positions && &collisions) {
        let pos = pos.expect("entity in positions-collisions join must have position component");
        let (x, y) = (pos.x, pos.y);
        if position_inside_rect(
            x,
            y,
            global_map.position.x,
            global_map.position.y,
            global_map.map.size().0,
            global_map.map.size().1,
        ) {
            let (t_x, t_y) = (global_map.position.x, global_map.position.y);
            global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
        }
    }

    for (pos, coll) in join!(&positions && &maps) {
        let pos = pos.expect("entity in positions-maps join must have position component");
        let coll = coll.expect("entity in positions-maps join must have collision map component");
        for i in 0..coll.size().0 as i32 {
            for j in 0..coll.size().1 as i32 {
                let (x, y) = (pos.x + i, pos.y + j);
                if coll.is_set(i as u32, j as u32)
                    && position_inside_rect(
                        x,
                        y,
                        global_map.position.x,
                        global_map.position.y,
                        global_map.map.size().0,
                        global_map.map.size().1,
                    )
                {
                    let (t_x, t_y) = (global_map.position.x, global_map.position.y);
                    global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
                }
            }
        }
    }
    Ok(())
}
