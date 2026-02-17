use std::sync::Arc;
// src/presentation/cli/cli_app.rs
use clap::{Parser, Command, Subcommand, value_parser};
use anyhow::Result;
use crate::modules::queue::storage::repositories::job_repository::JobRepository;
use crate::presentation::cli::commands::test_command;
use crate::presentation::cli::commands::queue_command;
use crate::presentation::cli::commands::index_command;
use crate::presentation::cli::commands::queue::queue_list_command;
use crate::queues::application::queue_service::JobService;
use crate::state::AppState;

// Аннотация #[derive(Parser)] генерирует код для парсинга аргументов
#[derive(Parser, Debug)]
// Атрибуты #[command] задают метаданные программы (берутся из Cargo.toml)
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Test commands
    #[command(name = "test")]
    Test(test_command::TestCommand),
    Index(index_command::IndexCommand),
    #[command(name = "queue:list")]
    QueueList(queue_list_command::QueueListCommand),
    Queue(queue_command::QueueCommand),
}

impl Commands {
    /// Выполнить подкоманду
    pub async fn execute(&self, state: AppState) -> Result<()> {
        match self {
            Commands::Test(cmd) => cmd.execute(state).await,
            Commands::Index(cmd) => cmd.execute(state).await,
            Commands::Queue(cmd) => cmd.execute(state).await,
            Commands::QueueList(cmd) => cmd.execute(state).await,
        }
    }
}
impl Args {
    /// Выполнить команду
    pub async fn execute(&self, state: AppState) -> Result<()> {
        if let Some(command) = &self.command {
            command.execute(state).await
        } else {
            // Если команда не указана, показываем help или запускаем сервер по умолчанию
            println!("No command specified. Use --help for available commands.");
            Ok(())
        }
    }
}


