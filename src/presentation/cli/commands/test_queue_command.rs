use clap::{Parser, Subcommand};
use crate::state::AppState;
use anyhow::Result;
use chrono::Utc;

#[derive(Parser, Debug)]
pub (crate) struct TestQueueCommand {
    #[command(subcommand)]
    pub action: TestQueueAction,
}

#[derive(Subcommand, Debug)]
pub enum TestQueueAction {
    Send {
        #[arg(short, long)]
        name: String,
    },
}

impl TestQueueCommand {
    pub async fn execute(&self, state: AppState) -> anyhow::Result<()> {
        match &self.action {
            TestQueueAction::Send { name } => {
                let filename = format!("queue_files/{}.rs", Utc::now().format("%Y-%m-%d_%H-%M-%S"));
                println!("📁 Send to queue... {}", name);
                // Логика запуска всех тестов
                Ok(())
            }
        }
    }
}