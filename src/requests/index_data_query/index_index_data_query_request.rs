use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexIndexDataQueryRequest {
    // pub uid: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub sort: Option<String>,
    pub sort_by: Option<String>,
    #[serde(flatten)]
    pub filter: Option<HashMap<String, String>>,
}
