use crate::state::AppState;
use anyhow::Result;
use chrono::Utc;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct QueueListCommand {
}

impl QueueListCommand {
    pub async fn execute(&self, state: AppState) -> anyhow::Result<()> {
        println!("👉 Queue List");
        Ok(())
    }
}
