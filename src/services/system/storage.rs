use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionUsage {
    pub filesystem: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskSmartHealth {
    pub device_path: String,
    pub model_name: String,
    pub is_healthy: bool,
    pub temperature_celsius: Option<i32>,
    pub power_on_hours: Option<u64>,
}

pub struct StorageService;

impl StorageService {
    pub fn new() -> Self {
        Self
    }

    /// 取得磁碟分割區使用狀態
    pub fn get_partition_usages(&self) -> AppResult<Vec<PartitionUsage>> {
        todo!()
    }

    /// 透過 smartctl 查詢特定磁碟 S.M.A.R.T. 健康檢查狀態
    pub async fn query_smart_health(&self, _device_path: &str) -> AppResult<DiskSmartHealth> {
        todo!()
    }
}
