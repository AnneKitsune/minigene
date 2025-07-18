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
    pub const fn is_inside(&self, p: &Point) -> bool {
        position_inside_rect(
            p.x - self.position.x,
            p.y - self.position.y,
            0,
            0,
            self.map.size().0,
            self.map.size().1,
        )
    }
    /// Check `is_inside` before calling this.
    pub const fn relative_point(&self, p: &Point) -> (u32, u32) {
        (
            (p.x - self.position.x) as u32,
            (p.y - self.position.y) as u32,
        )
    }
}

/// Tracks the current running state of the engine
#[derive(Default, Clone)]
pub struct EngineRunning {
    /// Whether the engine should continue running (`true`) or exit (`false`)
    pub running: bool,
}

/// Random Number Generator
/// It is suggested to create it manually if it is important that the numbers can't be guessed
/// easily.
#[derive(Clone, Debug, new)]
pub struct RNG {
    /// The random number generator.
    pub rng: oorandom::Rand32,
}

impl Default for RNG {
    fn default() -> Self {
        Self {
            rng: oorandom::Rand32::new(1),
        }
    }
}
