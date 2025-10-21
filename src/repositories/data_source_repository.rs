pub struct DataSource {}

impl DataSource {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn get_all(&self) -> String {
        "Hello World".to_string()
    }
}
