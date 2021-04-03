use crate::*;
/// Consumes `MoveCameraEvent` to move all cameras in the world.
/// Events are cleared by this system.
pub fn move_camera_system(cameras: &Components<Camera>, points: &mut Components<Point>, events: &mut Vec<MoveCameraEvent>) {
    for ev in events.iter() {
        let (mut off_x, mut off_y) = match ev.direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            _ => (0, 0),
        };
        off_x *= ev.distance;
        off_y *= ev.distance;
        for (mut p, _) in join!(&mut points && &cameras) {
            p.as_mut().unwrap().x += off_x;
            p.as_mut().unwrap().y += off_y;
        }
    }
}
