use clap::Args;

/// Find all assertions having the given object.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        todo!();
    }
}
