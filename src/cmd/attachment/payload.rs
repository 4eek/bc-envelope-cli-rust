use clap::Args;
use bc_envelope::prelude::*;

use crate::utils::read_envelope;

/// Get the payload of the attachment.
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    /// The attachment envelope.
    attachment: Option<String>,
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> anyhow::Result<String> {
        let attachment = read_envelope(self.attachment.as_deref())?;
        Ok(attachment.attachment_payload()?.ur_string())
    }
}
