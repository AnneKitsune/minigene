use crate::*;

#[derive(new)]
pub struct Comp<T>(pub T);
impl<T: Send + Sync + 'static> Component for Comp<T> {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Component)]
pub struct Sprite {
    pub glyph: u16,
    pub fg: RGBA,
    pub bg: RGBA,
}

#[derive(Component)]
pub struct SpriteIndex(pub usize);

#[derive(Component, new)]
pub struct MultiSprite {
    pub tile: MultiTileSprite,
}

#[derive(Component, new)]
pub struct AiPath {
    pub path: NavigationPath,
}

#[derive(Component, new)]
pub struct AiDestination {
    pub target: Point,
}

#[derive(Component, new)]
pub struct GotoStraight {
    pub target: Point,
    pub speed: f32,
}

#[derive(Component, new)]
pub struct GotoEntity {
    pub entity: Entity,
    pub speed: f32,
}

pub struct GameSpeed(f32);

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed(1.0)
    }
}

/// Collision of a single tile entity
#[derive(Component)]
pub struct Collision;
/// Collision of a multi tile entity. Not necessarily colliding everywhere.
/// Can be both used as a global resource and as a component for individual entities.
#[derive(Component)]
pub struct CollisionMap {
    bitset: BitSet,
    width: u32,
    height: u32,
}

impl CollisionMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bitset: BitSet::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn set(&mut self, x: u32, y: u32) {
        self.bitset.add(self.index_of(x, y));
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        self.bitset.remove(self.index_of(x, y));
    }

    pub fn is_set(&self, x: u32, y: u32) -> bool {
        self.bitset.contains(self.index_of(x, y))
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn clear(&mut self) {
        self.bitset.clear();
    }

    pub fn index_of(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }

    pub fn position_of(&self, idx: u32) -> (u32, u32) {
        (idx % self.width, idx / self.width)
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

#[derive(new)]
pub struct Camera {
    pub position: Point,
    pub size: Point,
}

#[derive(Debug, Clone, Copy, Component)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

