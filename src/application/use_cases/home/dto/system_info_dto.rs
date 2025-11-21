use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuData{
    name: String,
    cpu_usages: HashMap<String, f32>,
}

impl CpuData {
    pub fn new(name: String, cpu_usages: HashMap<String,f32>) -> Self {
        Self {
            name,
            cpu_usages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoDto {
    pub cpu: CpuData,
    pub memory: String,
    pub disk: String,
    pub network: String,
}