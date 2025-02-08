use crate::*;

/// Get the euclidian distance between two points.
pub fn dist(p1: &Point, p2: &Point) -> f32 {
    ((p2.x as f32 - p1.x as f32).powf(2.0) + (p2.y as f32 - p1.y as f32).powf(2.0)).sqrt()
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
    size_x > 0
        && size_y > 0
        && pos_x >= rect_x
        && pos_y >= rect_y
        && pos_x < rect_x + size_x as i32
        && pos_y < rect_y + size_y as i32
}

/// Moves the 2d point by 1 in the given direction.
/// Only works for 2d directions (North, South, East, West)
pub fn move_position(old_position: &Point, dir: Direction) -> Point {
    match dir {
        Direction::North => Point::new(old_position.x, old_position.y - 1),
        Direction::South => Point::new(old_position.x, old_position.y + 1),
        Direction::East => Point::new(old_position.x + 1, old_position.y),
        Direction::West => Point::new(old_position.x - 1, old_position.y),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn distance() {
        assert_eq!(dist(&Point::new(0, 0), &Point::new(0, 0)), 0.0);
        assert_eq!(dist(&Point::new(0, 0), &Point::new(5, 0)), 5.0);
        assert_eq!(dist(&Point::new(0, 0), &Point::new(-5, 0)), 5.0);
        assert_eq!(dist(&Point::new(0, 0), &Point::new(0, 5)), 5.0);
        assert_eq!(dist(&Point::new(0, 0), &Point::new(0, -5)), 5.0);
        assert_eq!(dist(&Point::new(0, -5), &Point::new(0, 0)), 5.0);
        assert_eq!(dist(&Point::new(0, 0), &Point::new(3, 4)), 5.0);
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
