pub extern crate bracket_lib;
#[macro_use]
extern crate pushdown_automaton_macro;
#[macro_use]
extern crate specs_declaration;
pub extern crate game_features;
pub extern crate specs;
#[macro_use]
extern crate specs_derive;
pub extern crate hibitset;
pub extern crate shrev;
#[macro_use]
extern crate derive_new;

pub use bracket_lib::prelude::*;
pub use game_features::*;
pub use hibitset::BitSet;
pub use shrev::*;
pub use specs::prelude::*;

// macro re-export
pub use derive_new::*;
pub use specs_declaration::*;
pub use specs_derive::*;

mod dispatcher;

pub use crate::dispatcher::*;

use std::collections::HashMap;
use std::sync::Arc;

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
        let mut o = smallvec![];
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

pub fn dist(p1: &Point, p2: &Point) -> f32 {
    ((p2.x as f32 - p1.x as f32).powf(2.0) + (p2.y as f32 - p1.y as f32).powf(2.0)).sqrt()
}

#[derive(new)]
pub struct CollisionResource {
    pub map: CollisionMap,
    pub position: Point,
}

impl CollisionResource {
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

#[derive(new)]
pub struct Camera {
    pub position: Point,
    pub size: Point,
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

system!(CombineCollisionSystem, |positions: ReadStorage<
    'a,
    Point,
>,
                                 collisions: ReadStorage<
    'a,
    Collision,
>,
                                 maps: ReadStorage<
    'a,
    CollisionMap,
>,
                                 global_map: WriteExpect<
    'a,
    CollisionResource,
>| {
    global_map.map.clear();

    for (pos, _) in (&positions, &collisions).join() {
        let (x, y) = (pos.x, pos.y);
        if position_inside_rect(
            x,
            y,
            global_map.position.x,
            global_map.position.y,
            global_map.map.size().0,
            global_map.map.size().1,
        ) {
            let (t_x, t_y) = (global_map.position.x, global_map.position.y);
            global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
        }
    }

    for (pos, coll) in (&positions, &maps).join() {
        for i in 0..coll.size().0 as i32 {
            for j in 0..coll.size().1 as i32 {
                let (x, y) = (pos.x + i, pos.y + j);
                if coll.is_set(i as u32, j as u32)
                    && position_inside_rect(
                        x,
                        y,
                        global_map.position.x,
                        global_map.position.y,
                        global_map.map.size().0,
                        global_map.map.size().1,
                    )
                {
                    let (t_x, t_y) = (global_map.position.x, global_map.position.y);
                    global_map.map.set((x - t_x) as u32, (y - t_y) as u32);
                }
            }
        }
    }
});

system!(
    AiPathingSystem,
    |dests: ReadStorage<'a, AiDestination>,
     global_map: ReadExpect<'a, CollisionResource>,
     positions: ReadStorage<'a, Point>,
     paths: WriteStorage<'a, AiPath>| {
        for (pos, dest, path) in (&positions, &dests, &mut paths).join() {
            if pos.x == dest.target.x && pos.y == dest.target.y {
                continue;
            }
            // TODO Safety check for < 0 or out of map bounds
            let d = global_map.map.index_of(
                (pos.x - global_map.position.x) as u32,
                (pos.y - global_map.position.y) as u32,
            );
            let t = global_map.map.index_of(
                (dest.target.x - global_map.position.x) as u32,
                (dest.target.y - global_map.position.y) as u32,
            );
            let p = a_star_search(d, t, &global_map.map);
            path.path = p;
        }
    }
);

system!(AiMovementSystem, |positions: WriteStorage<'a, Point>,
                           paths: WriteStorage<'a, AiPath>,
                           global_map: ReadExpect<
    'a,
    CollisionResource,
>| {
    // doesn't handle two entities that want to go to the same tile.
    for (pos, path) in (&mut positions, &mut paths).join() {
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.map.position_of(dest as u32);
            *pos = Point::new(
                x as i32 + global_map.position.x,
                y as i32 + global_map.position.y,
            );
        }
    }
});

#[macro_export]
macro_rules! event_reader_res {
    ($name:ident, $ev_type:ty) => {
        pub struct $name(pub ReaderId<$ev_type>);
    };
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

pub fn move_position(old_position: &Point, dir: Direction) -> Point {
    match dir {
        Direction::North => Point::new(old_position.x, old_position.y - 1),
        Direction::South => Point::new(old_position.x, old_position.y + 1),
        Direction::East => Point::new(old_position.x + 1, old_position.y),
        Direction::West => Point::new(old_position.x - 1, old_position.y),
        _ => unimplemented!(),
    }
}

event_reader_res!(InputDriverRes, VirtualKeyCode);

system!(
    InputDriver<E: Clone + Send + Sync + 'static>,
    |keymap: Read<'a, HashMap<VirtualKeyCode, E>>,
     inputs: Read<'a, EventChannel<VirtualKeyCode>>,
     events: Write<'a, EventChannel<E>>,
     reader: WriteExpect<'a, InputDriverRes>| {
        for i in inputs.read(&mut reader.0) {
            if let Some(e) = keymap.get(i) {
                events.single_write(e.clone());
            }
        }
    }
);

pub fn render_sprites<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: ReadStorage<'a, Point>,
    multi_sprites: ReadStorage<'a, MultiSprite>,
    sprites: ReadStorage<'a, Sprite>,
) {
    for (pos, sprite) in (&positions, &multi_sprites).join() {
        sprite.tile.render(
            ctx,
            Point::new(pos.x - camera.position.x, pos.y - camera.position.y),
        );
    }
    for (pos, sprite) in (&positions, &sprites).join() {
        ctx.set(
            pos.x - camera.position.x,
            pos.y - camera.position.y,
            sprite.fg,
            sprite.bg,
            sprite.glyph,
        );
    }
}

state_machine!(StateMachine; State; world: &mut World, dispatcher: &mut Dispatcher<'static, 'static>, ctx: &mut BTerm);

pub fn mini_loop<I: State + 'static>(
    world: &mut World,
    dispatcher: &mut Box<dyn UnifiedDispatcher + 'static>,
    init_state: I,
) {
    let mut state_machine = StateMachine::new(init_state);
    loop {
        let mut input = INPUT.lock();
        for key in input.key_pressed_set().iter() {
            world
                .fetch_mut::<EventChannel<VirtualKeyCode>>()
                .single_write(*key);
        }
        dispatcher.run_now(world);
        world.maintain();
        std::thread::sleep(std::time::Duration::from_millis(8));
    }
}

pub fn mini_init(
    width: u32,
    height: u32,
    name: &str,
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
    mut world: World,
    //mut dispatcher_builder: DispatcherBuilder<'static, 'static>,
) -> (World, Box<dyn UnifiedDispatcher + 'static>, BTerm) {
    #[cfg(feature = "wasm")]
    web_worker::init_panic_hook();
    let context = BTermBuilder::new()
        .with_simple_console(width, height, "terminal8x8.png")
        .with_font("terminal8x8.png", 8, 8)
        .with_title(name)
        .with_vsync(false)
        .with_advanced_input(true)
        .build()
        .expect("Failed to build BTerm context.");
    //#[cfg(feature = "wasm")]
    //{
    //    dispatcher_builder = dispatcher_builder.with_pool(Arc::new(web_worker::default_thread_pool(None).expect("Failed to create web worker thread pool")));
    //}
    //let mut dispatcher = dispatcher_builder.build();
    //dispatcher.setup(&mut world);
    world.insert(EventChannel::<VirtualKeyCode>::new());
    (world, dispatcher, context)
}

#[cfg(test)]
mod tests {
    use crate::CollisionMap;
    #[test]
    fn collmap() {
        let mut m = CollisionMap::new(3, 3);
        m.set(2, 2);
        assert!(m.is_set(2, 2));
        assert_eq!(m.index_of(2, 2), 8);
        assert_eq!(m.position_of(8), (2, 2));
    }
}
