// src/presentation/cli/cli_app.rs
use clap::{Parser, Command, Subcommand, value_parser};
use anyhow::Result;
use crate::presentation::cli::commands::test_command;
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

}

impl Commands {
    /// Выполнить подкоманду
    pub async fn execute(&self, state: Option<AppState>) -> Result<()> {
        match self {
            Commands::Test(cmd) => cmd.execute(state).await,
        }
    }
}
impl Args {
    /// Выполнить команду
    pub async fn execute(&self, state: Option<AppState>) -> Result<()> {
        if let Some(command) = &self.command {
            command.execute(state).await
        } else {
            // Если команда не указана, показываем help или запускаем сервер по умолчанию
            println!("No command specified. Use --help for available commands.");
            Ok(())
        }
    }
}


