pub fn dist(p1: &Point, p2: &Point) -> f32 {
    ((p2.x as f32 - p1.x as f32).powf(2.0) + (p2.y as f32 - p1.y as f32).powf(2.0)).sqrt()
}

pub fn position_inside_rect(
    pos_x: i32,
    pos_y: i32,
    rect_x: i32,
    rect_y: i32,
    size_x: u32,
    size_y: u32,
) -> bool {
    pos_x >= rect_x
        && pos_y >= rect_y
        && pos_x < rect_x + size_x as i32
        && pos_y < rect_y + size_y as i32
}

pub fn move_position(old_position: &Point, dir: Direction) -> Point {
    match dir {
        Direction::North => Point::new(old_position.x, old_position.y - 1),
        Direction::South => Point::new(old_position.x, old_position.y + 1),
        Direction::East => Point::new(old_position.x + 1, old_position.y),
        Direction::West => Point::new(old_position.x - 1, old_position.y),
        _ => unimplemented!(),
    }
}
