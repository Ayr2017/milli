use crate::state::AppState;
use anyhow::Result;
use chrono::Utc;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct QueueCommand {
    #[command(subcommand)]
    pub action: QueueAction,
}

#[derive(Subcommand, Debug)]
pub enum QueueAction {
    Work {},
    List {},
    Empty{}
}

impl QueueCommand {
    pub async fn execute(&self, state: AppState) -> anyhow::Result<()> {
        match &self.action {
            QueueAction::List {} => {
                println!("📁 List of queue.");
                // Логика запуска всех тестов
                Ok(())
            }
            QueueAction::Empty {} => {
                println!("Empty command.");
                Ok(())
            }
            _ => {
                println!("📁 Command not found.");
                Ok(())
            }
        }
    }
}
