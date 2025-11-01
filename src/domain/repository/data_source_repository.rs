pub trait DataSourceRepository {
    fn get(&self, id: i32) -> Option<Self> where Self: Sized;
    fn all(&self) -> Vec<Self> where Self: Sized;
    fn store(&self, data: Self) -> Self;
    fn update(&self, id: i32, data: Self) -> Option<Self> where Self: Sized;
    fn delete(&self, id: i32) -> Option<Self> where Self: Sized;
    fn delete_all(&self) -> Vec<Self> where Self: Sized;
    fn count(&self) -> i32;
    
}