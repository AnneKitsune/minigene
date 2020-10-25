system!(
    AiPathingSystem,
    |dests: ReadStorage<'a, AiDestination>,
     global_map: ReadExpect<'a, CollisionResource>,
     positions: ReadStorage<'a, Point>,
     paths: WriteStorage<'a, AiPath>| {
        for (pos, dest, path) in (&positions, &dests, &mut paths).join() {
            if pos.x == dest.target.x && pos.y == dest.target.y {
                continue;
            }
            // TODO Safety check for < 0 or out of map bounds
            let d = global_map.map.index_of(
                (pos.x - global_map.position.x) as u32,
                (pos.y - global_map.position.y) as u32,
            );
            let t = global_map.map.index_of(
                (dest.target.x - global_map.position.x) as u32,
                (dest.target.y - global_map.position.y) as u32,
            );
            let p = a_star_search(d, t, &global_map.map);
            path.path = p;
        }
    }
);
