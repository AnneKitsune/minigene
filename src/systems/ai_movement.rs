use crate::*;

/// Uses a `CollisionResource` and a `AiPath` component to move an entity
/// one step towards the desired destination.
pub fn ai_movement_system(
    global_map: &Option<CollisionResource>,
    positions: &mut Components<Point>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    // doesn't handle two entities that want to go to the same tile.
    for (pos, path) in join!(&mut positions && &mut paths) {
        let pos = pos.unwrap();
        let path = path.unwrap();
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.as_ref().unwrap().map.position_of(dest as u32);
            *pos = Point::new(
                x as i32 + global_map.as_ref().unwrap().position.x,
                y as i32 + global_map.as_ref().unwrap().position.y,
            );
        }
    }
    Ok(())
}
