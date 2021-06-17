use crate::*;

pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Minigene Command Interpreter",
    author = "Joël Lupien <jojolepromain@gmail.com>",
    about = "Command Line Interpreter for Minigene Game Engine."
)]
pub struct Command<T> {
    /// The command to execute in the game.
    #[structopt(subcommand)]
    pub command: T,
}

impl Manager {
    /// Executes the subcommand on the provided world.
    pub fn exec(&self, world: &mut World) {
        self.command.execute(world);
    }
}

trait Executable {
    fn execute(&self, &mut World);
}
