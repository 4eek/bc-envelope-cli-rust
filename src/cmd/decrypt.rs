use clap::Args;

/// Decrypt the envelope's subject using the provided key.
#[derive(Debug, Args)]
pub struct CommandArgs {
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) {
        todo!();
    }
}
