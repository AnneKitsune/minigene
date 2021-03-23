use crate::*;

/// A direction towards one of the 3d axis.
#[allow(missing_docs)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

/// Checks if the given position is inside of the given rectangle.
pub fn position_inside_rect(
    pos_x: i32,
    pos_y: i32,
    rect_x: i32,
    rect_y: i32,
    size_x: u32,
    size_y: u32,
) -> bool {
    assert!(size_x > 0);
    assert!(size_y > 0);
    pos_x >= rect_x
        && pos_y >= rect_y
        && pos_x < rect_x + size_x as i32
        && pos_y < rect_y + size_y as i32
}

/// Moves the 2d point by 1 in the given direction.
/// Only works for 2d directions (North, South, East, West)
pub fn move_position<T: std::ops::Add<T>>(old_position: &Vector3<T>, dir: Direction, distance: T) -> Vector3<T> {
    match dir {
        Direction::North => Vector3::<T>::new(old_position.x, old_position.y - 1, old_position.z),
        Direction::South => Vector3::<T>::new(old_position.x, old_position.y + 1, old_position.z),
        Direction::East => Vector3::<T>::new(old_position.x + 1, old_position.y, old_position.z),
        Direction::West => Vector3::<T>::new(old_position.x - 1, old_position.y, old_position.z),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn distance() {
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(0, 0)), 0.0);
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(5, 0)), 5.0);
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(-5, 0)), 5.0);
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(0, 5)), 5.0);
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(0, -5)), 5.0);
        assert_eq!(dist(&Vector3::new(0, -5), &Vector3::new(0, 0)), 5.0);
        assert_eq!(dist(&Vector3::new(0, 0), &Vector3::new(3, 4)), 5.0);
    }
    #[test]
    fn check_inside_rect() {
        assert!(position_inside_rect(0, 0, 0, 0, 1, 1));
        assert!(!position_inside_rect(0, 0, 1, 1, 1, 1));
        assert!(position_inside_rect(5, 3, 0, 0, 10, 10));
        assert!(position_inside_rect(-10, -10, -15, -15, 10, 10));
        assert!(position_inside_rect(-1, -1, -2, -2, 2, 2));
        assert!(!position_inside_rect(-1, -1, -2, -2, 1, 1));
    }
}
