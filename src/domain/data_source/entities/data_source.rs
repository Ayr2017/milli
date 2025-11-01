use crate::domain::data_source::errors::data_source_error::DataSourceError;

#[derive(Debug, Clone)]
pub struct DataSource {
    pub name: String,
    pub host: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_path: String,
    pub database_name: String,
    pub database_type: String,
}


impl DataSource {
    pub fn new(
        name: String,
        host: String,
        database: String,
        username: String,
        password: String,
        port: u16,
        database_path: String,
        database_name: String,
        database_type: String,
    ) -> Result<Self, DataSourceError> {

        Ok(Self {
            name,
            host,
            database,
            username,
            password,
            port,
            database_path,
            database_name,
            database_type,
        })
    }
}