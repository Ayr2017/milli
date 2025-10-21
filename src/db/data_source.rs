use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite, query, query_as, FromRow};

#[derive(Debug, FromRow)]
pub struct DataSource {
    pub id: u32,
    pub name: String,
    pub host: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_path: String,
    pub database_name: String,
    pub database_type: String,
    pub created_at: String,
    pub updated_at: String,
}

impl DataSource {
    pub async fn store(&self, pool: &Pool<Sqlite>) -> Result<i32, anyhow::Error> {
        let sql = "INSERT INTO data_sources (name, host, database, username, password, port, database_path, database_name, database_type, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = query(sql)
            .bind(&self.name)
            .bind(&self.host)
            .bind(&self.database)
            .bind(&self.username)
            .bind(&self.password)
            .bind(self.port)
            .bind(&self.database_path)
            .bind(&self.database_name)
            .bind(&self.database_type)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(pool)
            .await
            .context("Не удалось добавить источник данных".to_string())?;

        let id = result.last_insert_rowid() as i32;

        println!("DataSource '{}' добавлен с ID: {}", self.name, id);

        Ok(id)
    }

    pub async fn get_all_data_sources(&self, pool: &Pool<Sqlite>) -> Result<Vec<DataSource>, anyhow::Error> {
        let data_sources = query_as::<_, DataSource>("SELECT * FROM data_sources")
            .fetch_all(pool)
            .await
            .context("Не удалось получить список источников данных".to_string())?;

        Ok(data_sources)
    }
}
