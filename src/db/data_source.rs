use anyhow::Context;
use rusqlite::{params, Connection};

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
    pub fn store(&self, conn: Connection) -> rusqlite::Result<i32, anyhow::Error> {
        let sql = "INSERT INTO data_sources (name, host, database, username, password, port, database_path, database_name, database_type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

        conn.execute(sql, params![
            self.name,
            self.host,
            self.database, 
            self.username,
            self.password,
            self.port,
            self.database_path,
            self.database_name,
            self.database_type,
            self.created_at,
            self.updated_at
        ])
            .context("Не удалось добавить источник данных".to_string())?;

        let id = conn.last_insert_rowid() as i32;

        println!("DataSource '{}' добавлен с ID: {}", self.name, id);

        Ok(id)
    }
    
    pub fn get_all_data_sources(&self, conn: Connection) -> rusqlite::Result<Vec<DataSource>, anyhow::Error> {
        let mut stmt = conn.prepare("SELECT * FROM data_sources")?;
        
        let data_iter = stmt.query_map(params![], |row| {
            Ok(DataSource {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                host: row.get(2).unwrap(),
                database: row.get(3).unwrap(),
                username: row.get(4).unwrap(),
                password: row.get(5).unwrap(),
                port: row.get(6).unwrap(),
                database_path: row.get(7).unwrap(),
                database_name: row.get(8).unwrap(),
                database_type: row.get(9).unwrap(),
                created_at: row.get(10).unwrap(),
                updated_at: row.get(11).unwrap(),
            })
        }).context("Не удалось получить список источников данных".to_string())?;
        
        Ok(data_iter.map(|r| r.unwrap()).collect())
    }
        
}