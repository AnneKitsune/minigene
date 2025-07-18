use crate::*;

/// Moves an entity one tile towards the target position, regardless of collisions.
///
/// # Panics
/// This function panics if any entity is encountered that has a `GotoStraight` component without
/// a `Point` component, or vice versa. This situation is considered unrecoverable and indicates a
/// problem in entity setup or component management.
///
/// # Errors
/// FIXME: Currently, this function does not return any errors. In the future, we should handle component mismatches more gracefully.
pub fn goto_straight_system(
    gotos: &Components<GotoStraight>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for (p, goto) in join!(&mut positions && &gotos) {
        let p = p.expect("Expected Point component to exist for GotoStraight entity");
        let goto = goto.expect("Expected GotoStraight component to exist for Point entity");
        for _i in 0..(goto.speed as usize) {
            let delta_x = goto.target.x - p.x;
            let delta_y = goto.target.y - p.y;
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
    Ok(())
}
