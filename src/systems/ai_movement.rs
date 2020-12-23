use crate::*;

pub fn AiMovementSystem(positions: &mut Components<Point>,
                           paths: &mut Components<AiPath>,
                           global_map: &Option<CollisionResource>) {
    // doesn't handle two entities that want to go to the same tile.
    for (pos, path) in join!(&mut positions && &mut paths){
        let pos = pos.unwrap();
        let path = path.unwrap();
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.as_ref().unwrap().map.position_of(dest as u32);
            *pos = Point::new(
                x as i32 + global_map.as_ref().unwrap().position.x,
                y as i32 + global_map.as_ref().unwrap().position.y,
            );
        }
    }
}
