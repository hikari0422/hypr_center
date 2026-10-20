use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BluetoothDeviceInfo {
    pub address: String,
    pub name: Option<String>,
    pub is_paired: bool,
    pub is_connected: bool,
    pub rssi: Option<i16>,
}

pub struct BluetoothService;

impl BluetoothService {
    pub async fn new() -> AppResult<Self> {
        Ok(Self)
    }

    /// 檢查藍牙適配器電源狀態
    pub async fn is_adapter_powered(&self) -> AppResult<bool> {
        todo!()
    }

    /// 開啟或關閉藍牙適配器電源
    pub async fn set_adapter_powered(&self, _powered: bool) -> AppResult<()> {
        todo!()
    }

    /// 開始探索/掃描附近藍牙裝置
    pub async fn start_discovery(&self) -> AppResult<()> {
        todo!()
    }

    /// 停止掃描
    pub async fn stop_discovery(&self) -> AppResult<()> {
        todo!()
    }

    /// 連接至指定藍牙裝置
    pub async fn connect_device(&self, _address: &str) -> AppResult<()> {
        todo!()
    }

    /// 斷開指定藍牙裝置
    pub async fn disconnect_device(&self, _address: &str) -> AppResult<()> {
        todo!()
    }

    /// 移除/取消配對裝置
    pub async fn remove_device(&self, _address: &str) -> AppResult<()> {
        todo!()
    }
}
