use crate::*;

pub fn ai_pathing_system(
    dests: &Components<AiDestination>,
    global_map: &Option<CollisionResource>,
    positions: &Components<Point>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    for (pos, dest, path) in join!(&positions && &dests && &mut paths) {
        let pos = pos.unwrap();
        let dest = dest.unwrap();
        let path = path.unwrap();
        let global_map = global_map.as_ref().unwrap();
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
    Ok(())
}
