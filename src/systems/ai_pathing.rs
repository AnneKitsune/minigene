use crate::*;

pub fn ai_pathing_system(
    entities: &Entities,
    dests: &Components<AiDestination>,
    global_map: &Option<CollisionResource>,
    positions: &Components<Point>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    for (e, pos, dest) in join!(&entities && &positions && &dests) {
        let pos = pos.unwrap();
        let dest = dest.unwrap();
        let mut path = AiPath::default();
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
        paths.insert(e.unwrap(), path);
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
        let steps = paths.get(e).unwrap().path.steps.clone();
        assert_eq!(steps.len(), 3);
        assert_eq!(
            steps,
            vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
                .into_iter()
                .map(|p| global_map
                    .as_ref()
                    .unwrap()
                    .map
                    .index_of(p.x as u32, p.y as u32) as usize)
                .collect::<Vec<_>>()
        );
    }
}
