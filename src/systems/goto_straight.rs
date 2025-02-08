use crate::*;

/// Moves an entity one tile towards the target position, regardless of collisions.
pub fn goto_straight_system(
    gotos: &Components<GotoStraight>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for (p, goto) in join!(&mut positions && &gotos) {
        let p = p.unwrap();
        let goto = goto.unwrap();
        // TODO improve when we have a Time struct
        for _i in 0..(goto.speed as usize) {
            let delta_x = goto.target.x - p.x;
            let delta_y = goto.target.y - p.y;
            if delta_x.abs() >= delta_y.abs() {
                if delta_x > 0 {
                    p.x += 1;
                } else if delta_x < 0 {
                    p.x -= 1;
                }
            } else {
                if delta_y > 0 {
                    p.y += 1;
                } else if delta_y < 0 {
                    p.y -= 1;
                }
            }
        }
    }
    Ok(())
}
