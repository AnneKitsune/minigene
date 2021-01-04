use crate::*;

/// Holds the collision map used to calculate movements, ai pathing and collisions
/// between entities and the map.
#[derive(new)]
pub struct CollisionResource {
    /// The inner `CollisionMap`.
    pub map: CollisionMap,
    /// The position at which the `CollisionMap` is located.
    pub position: Point,
}

impl CollisionResource {
    /// Checks whether the position is located inside of the `CollisionMap`.
    pub fn is_inside(&self, p: &Point) -> bool {
        position_inside_rect(
            p.x - self.position.x,
            p.y - self.position.y,
            0,
            0,
            self.map.size().0,
            self.map.size().1,
        )
    }
    /// Check is_inside before calling this.
    pub fn relative_point(&self, p: &Point) -> (u32, u32) {
        (
            (p.x - self.position.x) as u32,
            (p.y - self.position.y) as u32,
        )
    }
}

/// Sets the game speed multiplier.
pub struct GameSpeed(pub f32);

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed(1.0)
    }
}
