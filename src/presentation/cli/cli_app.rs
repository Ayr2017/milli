// src/presentation/cli/cli_app.rs
use clap::{Parser, Command, Subcommand};
use anyhow::Result;
use crate::state::AppState;

// Аннотация #[derive(Parser)] генерирует код для парсинга аргументов
#[derive(Parser, Debug)]
// Атрибуты #[command] задают метаданные программы (берутся из Cargo.toml)
#[command(version, about, long_about = None)]
pub struct Args {
    // По умолчанию поле становится позиционным аргументом
    pub name: Option<String>,
}