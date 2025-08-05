use crate::*;
use uuidmap::Table;

/// Calculates a path from an entity's current position towards its `GotoStraight` target
/// and stores it in the `AiPath` component.
///
/// # Panics
/// This function may panic under the following circumstances:
/// - If the `global_map` resource is `None` when needed
/// - If expected component tables are missing
pub fn ai_pathing_system(
    gotos: &Table<GotoStraight>,
    global_map: &Option<CollisionResource>,
    positions: &Table<Point>,
    paths: &mut Table<AiPath>,
) {
    for goto in gotos.values() {
        // Get current position from positions table
        let Some(current_point) = positions.get(goto.current_position_id) else {
            continue;
        };

        // Calculate path
        let global_map = global_map
            .as_ref()
            .expect("global_map resource is required for ai_pathing_system");
        let start = Point::new(
            current_point.x - global_map.position.x,
            current_point.y - global_map.position.y,
        );
        let Some(dest) = positions.get(goto.destination_id) else {
            continue;
        };

        let target = Point::new(
            dest.x - global_map.position.x,
            dest.y - global_map.position.y,
        );

        if let Some(path) = astar(start, target, &global_map.map) {
            paths.add_with_key(goto.ai_path, AiPath { path: path.clone() });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use uuidmap::Table;

    #[test]
    fn correct_path() {
        let mut gotos = Table::<GotoStraight>::default();
        let mut positions = Table::<Point>::default();
        let mut paths = Table::<AiPath>::default();
        let global_map = Some(CollisionResource::new(
            CollisionMap::new(10, 10),
            Point::new(0, 0),
        ));

        let e = 1; // arbitrary entity id
                   // Set destination position for id 999
        positions.add_with_key(999, Point::new(1, 3));
        gotos.add_with_key(
            e,
            GotoStraight {
                destination_id: 999,
                current_position_id: e,
                ai_path: e,
            },
        );
        positions.add_with_key(e, Point::new(1, 1));

        ai_pathing_system(&gotos, &global_map, &positions, &mut paths);
        let steps = &paths.get(e).unwrap().path.path;
        assert_eq!(steps.len(), 3);
        assert_eq!(
            steps,
            &vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
        );
    }
}
