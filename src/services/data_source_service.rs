use anyhow::Error;
use crate::database::Database;
use crate::models::data_source::DataSource;

pub struct DataSourceService {
    pub connection: Database,
}




impl DataSourceService {
    
    pub fn new(connection: Database) -> DataSourceService {
        return DataSourceService{
            connection,
        };
    }
    pub fn test_data_source(data_source: DataSource) -> Result<String, Error> {
        return Ok("test".to_string());
    }
}