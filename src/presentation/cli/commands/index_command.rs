use crate::state::AppState;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::application::use_cases::index::index_check_use_case::IndexCheckUseCase;
use crate::presentation::cli::commands::test_command::{TestAction, TestCommand};

#[derive(Parser, Debug)]
pub(crate) struct IndexCommand {
    #[command(subcommand)]
    pub action: IndexAction,
}

#[derive(Subcommand, Debug)]
pub enum IndexAction {
    Check {
        #[arg(short, long)]
        uid: String,
    },
}

impl IndexCommand {
    pub async fn execute(&self, state: AppState) -> Result<()> {
        match &self.action {
            IndexAction::Check { uid } => {
                println!("{} {}", "☄️ Start checking index: ".purple(), uid.purple());
                IndexCheckUseCase::new(state).execute(uid.to_string());
                Ok(())
            }
        }
    }
}