use clap::Args;

/// Remove an assertion with the given predicate and object from the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        todo!();
    }
}