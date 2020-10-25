system!(CombineCollisionSystem, |positions: ReadStorage<
    'a,
    Point,
>,
                                 collisions: ReadStorage<
    'a,
    Collision,
>,
                                 maps: ReadStorage<
    'a,
    CollisionMap,
>,
                                 global_map: WriteExpect<
    'a,
    CollisionResource,
>| {
    global_map.map.clear();

    for (pos, _) in (&positions, &collisions).join() {
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

    for (pos, coll) in (&positions, &maps).join() {
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
});
