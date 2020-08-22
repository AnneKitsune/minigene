use super::UnifiedDispatcher;
use specs::prelude::*;

#[macro_export]
macro_rules! dispatcher {
    (
        $world:ident,
        $(
            (
                $type:ty,
                $name:expr,
                $deps:expr$(,)?
            )
        ),*
    ) => {
        //fn new_dispatch(world: &mut World) -> Box<dyn UnifiedDispatcher + 'static> {
        {
            let mut dispatcher = DispatcherBuilder::new()
                $(
                    .with(<$type>::default(), $name, $deps)
                )*
                .build();

            dispatcher.setup(&mut $world);
            let dispatch = MultiThreadedDispatcher {
                dispatcher,
            };

            Box::new(dispatch)
        }
    };
}

pub struct MultiThreadedDispatcher {
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl<'a> UnifiedDispatcher for MultiThreadedDispatcher {
    fn run_now(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
    }
}
