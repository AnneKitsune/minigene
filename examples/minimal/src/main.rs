use minigene::*;

/// The data passed to states in this game.
pub struct GameData {
    /// The world containing our resources, entities and components.
    pub world: World,
    /// The dispatcher containing `System`s. It is also used to execute them using
    /// the data contained in `World`.
    pub dispatcher: Dispatcher,
}

/// The default state of our game.
pub struct DefaultState;
impl State<GameData> for DefaultState {
    /// Called every game frame.
    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        // Run the systems contained in the dispatcher using data contained in world.
        data.dispatcher
            .run_seq(&mut data.world)
            .expect("One of the systems returned an error!");
        // Stay in this state; perform no transition.
        StateTransition::None
    }
}

fn main() {
    let mut world = World::default();
    let dispatcher = DispatcherBuilder::new().build(&mut world);
    {
        let mut term = world.get_mut_or_default::<Terminal>();
        *term = Terminal::new();
        term.clear();
        term.flush();
    }
    let gd = GameData { world, dispatcher };

    // Create the core engine.
    let mut engine = Engine::new(DefaultState, gd, |_, _| {}, 60.0);
    // Run the engine until we decide to quit.
    engine.engine_loop();
}
