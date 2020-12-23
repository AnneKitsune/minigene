use crate::*;

pub fn combine_collision_system(
    positions: &Components<Point>,
    collisions: &Components<Collision>,
    maps: &Components<CollisionMap>,
    global_map: &mut Option<CollisionResource>,
) {
    let global_map = global_map.as_mut().unwrap();

    global_map.map.clear();

    for (pos, _) in join!(&positions && &collisions) {
        let pos = pos.unwrap();
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
        let pos = pos.unwrap();
        let coll = coll.unwrap();
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
}
