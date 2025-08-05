use crate::*;

pub use crossterm::style::Color;
use hibitset::BitSet as HBitSet;
use std::collections::HashSet;

// TODO convert to tables

/// Represents a position in 2D space with integer coordinates.
///
/// Used for entities, camera positions, and rendering.
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    /// The x-coordinate (horizontal axis, increases rightward relative to the screen)
    pub x: i32,
    /// The y-coordinate (vertical axis, increases upward relative to the screen)
    pub y: i32,
}

impl Point {
    /// Creates a new `Point` from the given coordinates.
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
    /// Reference to a Point table row for position
    pub point_id: u128,
    /// Whether this sprite collides
    pub has_collision: bool,
}

/// A text-based sprite that is multiple tiles wide/high.
#[derive(new)]
pub struct MultiSprite {
    /// The ASCII characters to render (stored row-major, length = width * height)
    pub ascii: String,
    /// The width of the sprite in tiles
    pub width: u32,
    /// The height of the sprite in tiles
    pub height: u32,
    /// Foreground colors for each tile (length = width * height)
    pub fg: Vec<Color>,
    /// Background colors for each tile (length = width * height)
    pub bg: Vec<Color>,
    /// Reference to a Point table row for position
    pub point_id: u128,
    /// Reference to a `CollisionMap` table row
    pub collision_map: u128,
}

/// The path calculated by the Ai that it will follow.
#[derive(new, Default)]
pub struct AiPath {
    /// The path.
    pub path: Path,
}

/// The target terminal id to render to.
pub struct RenderTarget(pub usize);

/// Indicates that the ai should move towards a destination point.
#[derive(new)]
pub struct GotoStraight {
    /// Reference to a destination Point table row
    pub destination_id: u128,
    /// Reference to an origin Point table row
    pub current_position_id: u128,
    /// Reference to an `AiPath` table row
    pub ai_path: u128,
}

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

    /// Checks if the given coordinates are within the collision map boundaries.
    pub const fn is_inbound(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

/// Used to change the visible space of the world on screen.
/// TODO move to resources.rs
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

// TODO move somewhere else. This is not a component.
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

/// Everything that can be seen from a specific position.
/// Runtime only.
#[derive(new, Default)]
pub struct Viewshed {
    /// Reference to a Point table row for viewer position
    pub from_point_id: u128,
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
