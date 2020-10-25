pub use crate::*;

system!(AiMovementSystem, |positions: WriteStorage<'a, Point>,
                           paths: WriteStorage<'a, AiPath>,
                           global_map: ReadExpect<
    'a,
    CollisionResource,
>| {
    // doesn't handle two entities that want to go to the same tile.
    for (pos, path) in (&mut positions, &mut paths).join() {
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.map.position_of(dest as u32);
            *pos = Point::new(
                x as i32 + global_map.position.x,
                y as i32 + global_map.position.y,
            );
        }
    }
});
