use clap::Args;

/// (DEFAULT) Add an assertion with the given predicate and object to the given envelope.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
