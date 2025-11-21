use std::collections::HashMap;
use crate::application::use_cases::home::dto::system_info_dto::{CpuData, SystemInfoDto};
use sysinfo::{Cpu, System};
use tokio::time::{Duration};

pub struct GetSystemInfoUseCase {
    
}

impl GetSystemInfoUseCase {
    
    pub async fn new() -> Self {
        Self {}
    }
    pub async fn execute(&self) -> Result<SystemInfoDto, anyhow::Error> {
        let cpu = self.get_cpu().await?;
        println!("cpu: {:?}", cpu);
        Ok(SystemInfoDto {
            cpu,
            memory: "".to_string(),
            disk: "".to_string(),
            network: "".to_string(),
        })
    }

    async fn get_cpu(&self) -> Result<CpuData, anyhow::Error> {
        let mut sys = System::new_all(); // Инициализируем с полной информацией
        
        // Обновляем информацию о CPU
        sys.refresh_cpu_all();
        
        // Ждем немного для сбора данных о загрузке
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // Еще раз обновляем для получения актуальных данных о загрузке
        sys.refresh_cpu_all();
        
        let cpus = sys.cpus();
        let mut cpu_usages = HashMap::new();
        
        for cpu in cpus {
            cpu_usages.insert(cpu.name().to_string(), cpu.cpu_usage());
        }
        
        Ok(CpuData::new("cpu".to_string(), cpu_usages))
    }
    
    async fn get_memory() -> Result<String, anyhow::Error> {
        Ok("".to_string())
    }
    async fn get_disk() -> Result<String, anyhow::Error> {
        Ok("".to_string())
    }
}