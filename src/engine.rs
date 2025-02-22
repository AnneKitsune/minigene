use crate::resources::EngineRunning;
use planck_ecs::{DispatcherBuilder, System, World};

/// Run the engine using a single global dispatcher.
pub fn run(systems: Vec<System>) {
    let mut world = World::default();
    {
        let mut running = world.get_mut_or_default::<EngineRunning>();
        running.running = true;
    }
    let mut dispatcher_builder = DispatcherBuilder::default();
    for sys in systems {
        dispatcher_builder = dispatcher_builder.add_system(sys);
    }
    let mut dispatcher = dispatcher_builder.build(&mut world);

    while world.get::<EngineRunning>().unwrap().running {
        let _ = dispatcher.run_seq(&world);
    }
}
