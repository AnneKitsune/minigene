use crate::*;

/// Consumes `MoveCameraEvent` to move the camera resource.
/// Events are cleared by this system.
pub fn move_camera_system(camera: &mut Camera, events: &mut Vec<MoveCameraEvent>) -> SystemResult {
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
        camera.position.x += off_x;
        camera.position.y += off_y;
    }
    events.clear();
    Ok(())
}
