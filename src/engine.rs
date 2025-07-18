use crate::resources::EngineRunning;
use planck_ecs::{Dispatcher, DispatcherBuilder, System, World};

fn build_dispatcher(world: &mut World, systems: Vec<System>) -> Dispatcher {
    let mut dispatcher_builder = DispatcherBuilder::default();
    for sys in systems {
        dispatcher_builder = dispatcher_builder.add_system(sys);
    }
    dispatcher_builder.build(world)
}

/// Run the engine using a single global dispatcher.
///
/// # Panics
///
/// Panics if the initial `EngineRunning` resource is not present after setup.
pub fn run(init_systems: Vec<System>, run_systems: Vec<System>) {
    let mut world = World::default();
    {
        let mut running = world.get_mut_or_default::<EngineRunning>();
        running.running = true;
    }

    let mut init_dispatcher = build_dispatcher(&mut world, init_systems);
    let mut run_dispatcher = build_dispatcher(&mut world, run_systems);

    let _ = init_dispatcher.run_seq(&world);

    while world
        .get::<EngineRunning>()
        .expect("EngineRunning resource missing")
        .running
    {
        let _ = run_dispatcher.run_seq(&world);
    }
}
