use crate::*;

/// Moves an entity one tile towards the target position, regardless of collisions.
pub fn goto_straight_system(
    gotos: &Components<GotoStraight>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for (p, goto) in join!(&mut positions && &gotos) {
        let p = p.unwrap();
        let goto = goto.unwrap();
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
