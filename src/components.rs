use crate::*;

use std::collections::HashSet;

/// A single colored letter sprite.
pub struct Sprite {
    /// The char symbol displayed.
    pub glyph: u16,
    /// The foreground color.
    pub fg: RGBA,
    /// The background color.
    pub bg: RGBA,
}

/// The index of a 2d sprite. Created from `SpriteSheet`'s index.
pub struct SpriteIndex(pub usize);

/// A text-based sprite that is multiple tiles wide/high.
#[derive(new)]
pub struct MultiSprite {
    /// The tile.
    pub tile: MultiTileSprite,
}

/// The path calculated by the Ai that it will follow.
#[derive(new, Default)]
pub struct AiPath {
    /// The path.
    pub path: NavigationPath,
}

/// Indicates that the ai should calculate an AiPath from the current position
/// towards this destination.
#[derive(new)]
pub struct AiDestination {
    /// The destination position.
    pub target: Point,
}

/// Indicates that the ai should calculate an AiPath from the current position
/// towards this destination.
#[derive(new)]
pub struct GotoStraight {
    /// The destination position.
    pub target: Point,
    /// The speed at which the entity moves in tiles/second.
    pub speed: f32,
}

/// Indicates that the ai should calculate an AiPath from the current position
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
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Erase the collision map.
    pub fn clear(&mut self) {
        self.bitset.clear();
    }

    // pub(crate) fn index_of(&self, x: u32, y: u32) -> u32 {
    pub fn index_of(&self, x: u32, y: u32) -> u32 {
        let idx = y * self.width + x;
        assert!(idx <= self.width * self.height - 1);
        idx
    }

    // pub(crate) fn position_of(&self, idx: u32) -> (u32, u32) {
    pub fn position_of(&self, idx: u32) -> (u32, u32) {
        assert!(self.width > 0);
        assert!(self.height > 0);
        (idx % self.width, idx / self.width)
    }
}

impl Algorithm2D for CollisionMap {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for CollisionMap {
    fn is_opaque(&self, idx: usize) -> bool {
        self.bitset.contains(idx as u32)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut o = SmallVec::new();
        //println!("idx: {}", idx);
        // right
        if (idx % self.width as usize) < (self.width as usize - 1) {
            let n = idx + 1;
            if !self.is_opaque(n) {
                //println!("ADDING AT {},{}, while it is {} opaque.", self.position_of(idx as u32).0, self.position_of(idx as u32).1, self.is_opaque(idx));
                o.push((n, 1.0));
            }
        }
        // left
        if (idx % self.width as usize) > 0 {
            let n = idx - 1;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        // down
        if (idx / self.width as usize) < (self.height as usize - 1) {
            let n = idx + self.width as usize;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        // up
        if idx >= (self.width as usize) {
            let n = idx - self.width as usize;
            if !self.is_opaque(n) {
                o.push((n, 1.0));
            }
        }
        o
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (x1, y1) = self.position_of(idx1 as u32);
        let (x2, y2) = self.position_of(idx2 as u32);
        ((x2 as f32 - x1 as f32).powf(2.0) + (y2 as f32 - y1 as f32).powf(2.0)).sqrt()
    }
}

// TODO consider changing this to a component?
/// Used to change the visible space of the world on screen.
#[derive(new)]
pub struct Camera {
    /// The position of the camera.
    pub position: Point,
    /// The size in tiles that the camera can view.
    pub size: Point,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
            size: Point::new(80, 50),
        }
    }
}

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

/// Everything we can see from.
#[derive(new)]
pub struct Viewshed {
    /// Which tiles we can see.
    pub visible_tiles: HashSet<Point>,
}

impl Default for Viewshed {
    fn default() -> Self {
        Self {
            visible_tiles: HashSet::new(),
        }
    }
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
