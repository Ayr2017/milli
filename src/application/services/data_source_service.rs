use crate::domain::repository::data_source_repository::DataSourceRepository;

pub struct DataSourceService<R: DataSourceRepository> {
    repository: R,
}

impl <R: DataSourceRepository> DataSourceService <R> {
    pub fn new(repository: R) -> Self { Self {  repository  } }
}