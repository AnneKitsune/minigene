use crate::*;
use uuidmap::Table;

/// Combine entity's collision data (from Sprite and `MultiSprite`) into the global `CollisionResource`.
///
/// # Panics
/// Panics if:
/// - `global_map` is `None`
pub fn combine_collision_system(
    sprites: &Table<Sprite>,
    multi_sprites: &Table<MultiSprite>,
    maps: &Table<CollisionMap>,
    positions: &Table<Point>,
    global_map: &mut Option<CollisionResource>,
) {
    let global_map = global_map.as_mut().expect("global_map must be set");
    global_map.map.clear();

    // Process single-tile collisions from Sprite
    for sprite in sprites.values() {
        if !sprite.has_collision {
            continue;
        }
        let Some(pos) = positions.get(sprite.point_id) else {
            continue;
        };
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

    // Process multi-tile collisions from MultiSprite
    for multi_sprite in multi_sprites.values() {
        let Some(pos) = positions.get(multi_sprite.point_id) else {
            continue;
        };
        let Some(coll_map) = maps.get(multi_sprite.collision_map) else {
            continue;
        };

        for i in 0..coll_map.size().0 as i32 {
            for j in 0..coll_map.size().1 as i32 {
                let (x, y) = (pos.x + i, pos.y + j);
                if coll_map.is_set(i as u32, j as u32)
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
