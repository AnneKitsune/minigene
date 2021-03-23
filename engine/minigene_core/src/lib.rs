//! Core crate for minigene.
#![deny(missing_docs)]
//#[macro_use]
//extern crate pushdown_automaton_macro;

pub use game_clock::*;
pub use planck_ecs::*;

// macro re-export
pub use derive_new::*;
pub use game_state_machine::*;

use spin_sleep::LoopHelper;

const DEFAULT_FPS: f32 = 60.0;

trait Bundle {
    fn systems() -> Vec<System>;
    fn insert(builder: DispatcherBuilder) -> DispatcherBuilder {
        let mut builder = builder;
        for sys in Self::systems() {
            builder = builder.add_system(sys);
        }
        builder
    }
}

/// Runs the engine until the state machine quits.
pub fn mini_loop<SD, I: State<SD> + 'static>(
    mut state_data: SD,
    init_state: I,
    max_fps: f32,
) {
    let mut loop_helper = LoopHelper::builder().build_with_target_rate(max_fps);
    let mut state_machine = StateMachine::default();
    let mut time = Time::default();

    state_machine.push(Box::new(init_state), &mut state_data);
    while state_machine.is_running() {
        let delta = loop_helper.loop_start();
        {
            time.advance_frame(delta);
        }
        // state should be the one calling run_dispatcher in the on_update method.
        // TODO how to increment time?
        // TODO think about a way to remove the burden of calling run_dispatcher from the state?
        state_machine.update(&mut state_data);
        loop_helper.loop_sleep();
    }
}

/// Runs the provided dispatcher on the provided world.
pub fn run_dispatcher(
    world: &mut World,
    dispatcher: &mut Dispatcher,
) -> SystemResult {
    #[cfg(not(feature = "parallel"))]
    let result = dispatcher.run_seq(world);
    #[cfg(feature = "parallel")]
    let result = dispatcher.run_par(world);

    world.maintain();

    result
}

/// A simple loop driver using as many defaults as possible. Only really suitable
/// to run a single dispatcher for a single world.
pub fn mini_loop_simple (world: World, dispatcher: Dispatcher) {
    let mut loop_helper = LoopHelper::builder().build_with_target_rate(DEFAULT_FPS);
    let mut state_machine = StateMachine::default();
    let mut state_data = DefaultStateData {
        world,
        dispatcher,
    };
    let mut time = Time::default();

    state_machine.push(Box::new(DefaultState), &mut state_data);
    while state_machine.is_running() {
        let delta = loop_helper.loop_start();
        {
            time.advance_frame(delta);
        }
        // state should be the one calling run_dispatcher in the on_update method.
        // TODO how to increment time?
        // TODO think about a way to remove the burden of calling run_dispatcher from the state?
        state_machine.update(&mut state_data);
        run_dispatcher(&mut state_data.world, &mut state_data.dispatcher).expect("System execution failed.");
        loop_helper.loop_sleep();
    }
}

/// The default data sent to states.
pub struct DefaultStateData {
    /// The world.
    pub world: World,
    /// The dispatcher.
    pub dispatcher: Dispatcher,
}

impl Default for DefaultStateData {
    fn default() -> Self {
        let mut world = World::default();
        let dispatcher = DispatcherBuilder::default().build(&mut world);
        Self {
            world,
            dispatcher,
        }
    }
}

/// The default state transition.
pub type DefaultTrans = StateTransition<DefaultStateData>;

/// A default empty state.
pub struct DefaultState;
impl State<DefaultStateData> for DefaultState {}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_loop() {
        struct MyState;
        impl State<DefaultStateData> for MyState {
            fn update(&mut self, _state_data: &mut DefaultStateData) -> DefaultTrans {
                StateTransition::Quit
            }
        }
        mini_loop(DefaultStateData::default(), MyState, 1000.0);
    }
    #[test]
    fn bundle() {
        struct TestBundle;
        impl Bundle for TestBundle {
            fn systems() -> Vec<System> {
                vec![
                    (|| {Ok(())}).system(),
                    (|| {Ok(())}).system(),
                    (|| {println!("hello!"); Ok(())}).system(),
                ]
            }
        }
        let mut builder = DispatcherBuilder::default();
        #[allow(unused_assignments)]
        {
            builder = TestBundle::insert(builder);
        }
    }
}

