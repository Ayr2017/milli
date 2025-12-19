use crate::state::AppState;

pub struct IndexCheckUseCase {
    state: AppState,
}

impl IndexCheckUseCase {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
        }
    }
    
    pub fn execute(&self, uid: String) -> bool {
        let index = self.state.meilisearch_client.index(uid);
        println!("index: {:#?}", index);
        true
    }
}