use crate::state::AppState;
use anyhow::Result;
use chrono::Utc;
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser, Debug)]
pub(crate) struct QueueListCommand {
}

impl QueueListCommand {
    pub async fn execute(&self, state: AppState) -> anyhow::Result<()> {
        println!("{}", "📋 Last 10 Jobs".bright_blue().bold());
        println!();
        
        

        println!("👉 Queue List");
        Ok(())
    }
}
