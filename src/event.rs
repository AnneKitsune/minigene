use crate::*;

/// Moves all cameras in the world by `distance` towards `direction`.
/// Consumed by `move_camera_system`.
pub struct MoveCameraEvent {
    /// The direction in which to move the camera.
    pub direction: Direction,
    /// The distance that the camera should be moved.
    pub distance: i32,
}
