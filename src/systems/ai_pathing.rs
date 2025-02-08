use crate::*;

/// Calculates a path from the entity's current position towards the specified
/// `AiDestination` and inserts it in a `AiPath` component.
/// It uses a `CollisionResource` to take collisions into account.
pub fn ai_pathing_system(
    entities: &Entities,
    dests: &Components<AiDestination>,
    global_map: &Option<CollisionResource>,
    positions: &Components<Point>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    'query: for (e, pos, dest) in join!(&entities && &positions && &dests) {
        // Check if entity already has AIpath with the same destination:
        if let Some(existing_path) = paths.get(e.unwrap()) {
            let curr_dest = existing_path.path.path.last();
            let new_dest = Point::new(dest.unwrap().target.x, dest.unwrap().target.y);
            if curr_dest == Some(&new_dest) {
                continue 'query;
            }
        }
        let pos = pos.unwrap();
        let dest = dest.unwrap();
        let mut path = AiPath::default();
        let global_map = global_map.as_ref().unwrap();
        if pos.x == dest.target.x && pos.y == dest.target.y {
            continue;
        }
        // TODO Safety check for < 0 or out of map bounds
        let d = Point::new(pos.x - global_map.position.x, pos.y - global_map.position.y);
        let t = Point::new(
            dest.target.x - global_map.position.x,
            dest.target.y - global_map.position.y,
        );
        if let Some(p) = astar(d, t, &global_map.map) {
            path.path = p;
            paths.insert(e.unwrap(), path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn correct_path() {
        let mut entities = Entities::default();
        let mut dests = Components::<AiDestination>::default();
        let mut positions = Components::<Point>::default();
        let mut paths = Components::<AiPath>::default();
        let global_map = Some(CollisionResource::new(
            CollisionMap::new(10, 10),
            Point::new(0, 0),
        ));

        let e = entities.create();
        dests.insert(e, AiDestination::new(Point::new(1, 3)));
        positions.insert(e, Point::new(1, 1));

        ai_pathing_system(&entities, &dests, &global_map, &positions, &mut paths).unwrap();
        let steps = &paths.get(e).unwrap().path.path;
        assert_eq!(steps.len(), 3);
        assert_eq!(
            steps,
            &vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
        );
    }
}
