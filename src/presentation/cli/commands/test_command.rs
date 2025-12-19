use crate::state::AppState;
use anyhow::Result;
use clap::{Parser, Subcommand};

/**
 * Команда для примера
 */
#[derive(Parser, Debug)]
pub(crate) struct TestCommand {
    #[command(subcommand)]
    pub action: TestAction,
}

#[derive(Subcommand, Debug)]
pub enum TestAction {
    Print {
        #[arg(short, long)]
        name: String,
    },
}

impl TestCommand {
    pub async fn execute(&self, state: AppState) -> Result<()> {
        match &self.action {
            TestAction::Print { name } => {
                println!("🔄 Running all tests... {}", name);
                // Логика запуска всех тестов
                Ok(())
            }
        }
    }
}
