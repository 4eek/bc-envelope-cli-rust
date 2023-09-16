pub mod envelope;
pub mod predicate_object;

use clap::{Subcommand, Args};

/// Remove an assertion from the given envelope.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Envelope(envelope::CommandArgs),
    PredicateObject(predicate_object::CommandArgs),
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String, anyhow::Error> {
        match &self.command {
            SubCommands::Envelope(args) => args.exec(),
            SubCommands::PredicateObject(args) => args.exec(),
        }
    }
}
