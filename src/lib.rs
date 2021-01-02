pub extern crate bracket_lib;
#[macro_use]
extern crate pushdown_automaton_macro;
pub extern crate game_features;
pub extern crate hibitset;

#[cfg(feature = "terminal")]
extern crate crossterm;

pub use bracket_lib::prelude::{
    a_star_search, add_wasm_support, main_loop, to_cp437, BError, BTerm, BTermBuilder, BaseMap,
    GameState, MultiTileSprite, NavigationPath, Point, Rect, SmallVec, SpriteSheet, VirtualKeyCode,
    BLACK, BLUE, EMBED, GREEN, INPUT, RED, RGBA, WHITE, YELLOW,
};
pub use game_clock::*;
pub use game_features::*;
pub use hibitset::BitSet as HBitSet;
pub use stopwatch::*;

//pub use entity_component::*;
//pub use world_dispatcher::*;
pub use plank_ecs::*;

// macro re-export
pub use derive_new::*;

pub use spin_sleep::LoopHelper;

mod components;
mod macros;
mod render;
mod resources;
mod systems;
mod utils;

pub use self::components::*;
pub use self::macros::*;
pub use self::render::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::utils::*;

use std::collections::HashMap;

use std::fmt::Debug;
use std::hash::Hash;

state_machine!(StateMachine; State; world: &mut World, dispatcher: &mut Dispatcher, ctx: &mut BTerm);

/// Runs the engine until the state machine quits.
pub fn mini_loop<I: State + 'static>(
    world: &mut World,
    dispatcher: &mut Dispatcher,
    ctx: &mut BTerm,
    init_state: I,
    max_fps: f32,
) {
    let mut loop_helper = LoopHelper::builder().build_with_target_rate(max_fps);
    let mut state_machine = StateMachine::new(init_state);
    state_machine.start(world, dispatcher, ctx);
    while state_machine.is_running() {
        let delta = loop_helper.loop_start();
        {
            let mut time = world.get_mut::<Time>().unwrap();
            time.advance_frame(delta);
        }
        mini_frame(world, dispatcher, ctx, &mut state_machine);
        loop_helper.loop_sleep();
    }
}

/// Runs a single game frame and updates the state machine.
pub fn mini_frame(
    world: &mut World,
    dispatcher: &mut Dispatcher,
    ctx: &mut BTerm,
    state_machine: &mut StateMachine,
) {
    //#[cfg(not(feature = "wasm"))]
    //world.get_mut::<Stopwatch>().unwrap().start();

    let input = INPUT.lock();
    for key in input.key_pressed_set().iter() {
        world.get_mut::<Vec<VirtualKeyCode>>().unwrap().push(*key);
    }
    #[cfg(feature = "wasm")]
    dispatcher.run_seq(world).unwrap();
    #[cfg(not(feature = "wasm"))]
    dispatcher.run_par(world).unwrap();
    state_machine.update(world, dispatcher, ctx);
    world.maintain();

    //#[cfg(not(target_arch = "wasm32"))]
    //std::thread::sleep(std::time::Duration::from_millis(8));

    //#[cfg(not(feature = "wasm"))]
    //let elapsed = world.get::<Stopwatch>().elapsed();
    //#[cfg(feature = "wasm")]
    //let elapsed = std::time::Duration::from_millis(16);
    //time.advance_frame(elapsed);
    //#[cfg(not(feature = "wasm"))]
    //{
    //    let stopwatch = world.get_mut::<Stopwatch>().unwrap();
    //    stopwatch.stop();
    //    stopwatch.restart();
    //}
}

/// Initializes the engine structures.
pub fn mini_init(
    width: u32,
    height: u32,
    name: &str,
    #[allow(unused)] spritesheet: Option<SpriteSheet>,
    dispatcher: Dispatcher,
    world: World,
    //mut dispatcher_builder: DispatcherBuilder<'static, 'static>,
) -> (World, Dispatcher, BTerm) {
    #[cfg(feature = "terminal")]
    std::panic::set_hook(Box::new(|panic_info| {
        crossterm::terminal::disable_raw_mode().unwrap();
        let location = panic_info.location().unwrap();
        println!("Panic occured at {}:{}", location.file(), location.line());
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("Panic occured: {:?}", s);
        }
        //execute!(std::io::stdout(), crossterm::terminal::EnableLineWrap);
    }));
    #[cfg(feature = "wasm")]
    web_worker::init_panic_hook();
    let mut context = BTermBuilder::new();
    //#[cfg(not(feature = "opengl"))]
    {
        context = context.with_simple_console(width, height, "terminal8x8.png");
    }
    #[cfg(feature = "opengl")]
    {
        if let Some(ss) = spritesheet {
            context = context.with_sprite_sheet(ss);
            context = context.with_sprite_console(width, height, 0);
        } else {
            println!("Using opengl mode without a spritesheet!");
        }
    }

    let context = context
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

    #[cfg(not(target_arch = "wasm32"))]
    init_thread_pool().unwrap();

    (world, dispatcher, context)
}

/*#[cfg(test)]
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
}*/
