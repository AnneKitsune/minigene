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
        //fn new_dispatch(_world: &mut World) -> Box<dyn UnifiedDispatcher + 'static> {
        {
            let mut dispatch = SingleThreadedDispatcher {
                systems: Vec::new()
            };

            $(
                dispatch.systems.push(Box::new(<$type>::default()));
            )*

            Box::new(dispatch)
        }
    };
}

pub struct SingleThreadedDispatcher {
    pub systems: Vec<Box<dyn RunNow<'static>>>,
}

impl UnifiedDispatcher for SingleThreadedDispatcher {
    fn run_now(&mut self, ecs: *mut World) {
        unsafe {
            for sys in self.systems.iter_mut() {
                sys.run_now(&*ecs);
            }
        }
    }
}
