use crate::*;

pub use crossterm::style::Color;
use std::collections::HashSet;

// TODO convert to tables

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A single colored letter sprite.
pub struct Sprite {
    /// The char symbol displayed.
    pub glyph: char,
    /// The foreground color.
    pub fg: Color,
    /// The background color.
    pub bg: Color,
}

/// A text-based sprite that is multiple tiles wide/high.
#[derive(new)]
pub struct MultiSprite {
    pub ascii: String,
    pub width: u32,
    pub height: u32,
    pub fg: Vec<Color>,
    pub bg: Vec<Color>,
}

/// The path calculated by the Ai that it will follow.
#[derive(new, Default)]
pub struct AiPath {
    /// The path.
    pub path: Path,
}

/// The target terminal id to render to.
pub struct RenderTarget(pub usize);

/// Indicates that the ai should calculate an `AiPath` from the current position
/// towards this destination.
#[derive(new)]
pub struct AiDestination {
    /// The destination position.
    pub target: Point,
}

/// Indicates that the ai should calculate an `AiPath` from the current position
/// towards this destination.
#[derive(new)]
pub struct GotoStraight {
    /// The destination position.
    pub target: Point,
    /// The speed at which the entity moves in tiles/second.
    pub speed: f32,
}

/// Indicates that the ai should calculate an `AiPath` from the current position
/// towards this entity's position.
#[derive(new)]
pub struct GotoEntity {
    /// The destination entity we are trying to reach.
    pub entity: Entity,
    /// The speed at which the entity moves in tiles/second.
    pub speed: f32,
}

/// Collision of a single tile entity
pub struct Collision;
/// Collision of a multi tile entity. Not necessarily colliding everywhere.
/// Can be both used as a global resource and as a component for individual entities.
#[derive(Debug, Clone)]
pub struct CollisionMap {
    bitset: HBitSet,
    width: u32,
    height: u32,
}

impl CollisionMap {
    /// Create a new collision map.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bitset: HBitSet::with_capacity(width * height),
            width,
            height,
        }
    }

    /// Enable collision at the given position.
    pub fn set(&mut self, x: u32, y: u32) {
        self.bitset.add(self.index_of(x, y));
    }

    /// Disable collision at the given position.
    pub fn unset(&mut self, x: u32, y: u32) {
        self.bitset.remove(self.index_of(x, y));
    }

    /// Checks if collision is enabled at the given position.
    pub fn is_set(&self, x: u32, y: u32) -> bool {
        self.bitset.contains(self.index_of(x, y))
    }

    /// Gives the size of the collision map.
    pub const fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Erase the collision map.
    pub fn clear(&mut self) {
        self.bitset.clear();
    }

    // pub(crate) fn index_of(&self, x: u32, y: u32) -> u32 {
    /// Gives the index of the given (x, y) position.
    ///
    /// # Panics
    /// Panics if the position is out of bounds.
    pub fn index_of(&self, x: u32, y: u32) -> u32 {
        let idx = y * self.width + x;
        assert!(
            idx < self.width * self.height,
            "Position ({}, {}) is out of bounds for collision map of size ({}, {})",
            x,
            y,
            self.width,
            self.height
        );
        idx
    }

    // pub(crate) fn position_of(&self, idx: u32) -> (u32, u32) {
    /// Gives the (x, y) position of the given index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds or if the collision map has zero width or height.
    pub fn position_of(&self, idx: u32) -> (u32, u32) {
        assert!(self.width > 0, "CollisionMap width must be greater than 0");
        assert!(
            self.height > 0,
            "CollisionMap height must be greater than 0"
        );
        (idx % self.width, idx / self.width)
    }

    pub const fn is_inbound(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

// TODO consider changing this to a component or keep as a resource?
/// Used to change the visible space of the world on screen.
#[derive(new, Clone, Debug)]
pub struct Camera {
    /// The position of the camera in the world.
    pub position: Point,
    /// The position of the camera on screen.
    pub screen_position: Point,
    /// The size in tiles that the camera can view.
    pub size: Point,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
            screen_position: Point::new(0, 0),
            size: Point::new(80, 50),
        }
    }
}

/// A direction towards one of the 3d axis.
#[allow(missing_docs, reason = "trivial names")]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

/// Everything we can see from.
#[derive(new, Default)]
pub struct Viewshed {
    /// Which tiles we can see.
    pub visible_tiles: HashSet<Point>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn collision_map_set_unset_clear() {
        let mut map = CollisionMap::new(5, 5);
        assert!(!map.is_set(3, 3));
        map.set(3, 3);
        assert!(map.is_set(3, 3));
        map.unset(3, 3);
        assert!(!map.is_set(3, 3));
        map.set(3, 3);
        map.clear();
        assert!(!map.is_set(3, 3));
    }
    #[test]
    fn small_map() {
        let _ = CollisionMap::new(0, 0);
        let mut map = CollisionMap::new(1, 1);
        map.set(0, 0);
        assert!(map.is_set(0, 0));
    }
    #[test]
    fn huge_map() {
        let mut map = CollisionMap::new(1000, 1000);
        map.set(999, 999);
    }

    #[test]
    #[should_panic]
    fn small_map_out_of_bounds() {
        let mut map = CollisionMap::new(0, 0);
        map.set(0, 0);
        assert!(map.is_set(0, 0));
    }
    #[test]
    #[should_panic]
    fn big_map_out_of_bounds() {
        let mut map = CollisionMap::new(1000, 1000);
        map.set(1000, 1000);
        assert!(map.is_set(1000, 1000));
        map.set(9999, 1000);
        assert!(map.is_set(9999, 1000));
    }
}
