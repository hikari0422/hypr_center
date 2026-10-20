use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPointInfo {
    pub ssid: String,
    pub signal_strength: u8,
    pub is_secure: bool,
    pub is_connected: bool,
}

pub struct NetworkService;

impl NetworkService {
    pub async fn new() -> AppResult<Self> {
        Ok(Self)
    }

    /// 檢查 Wi-Fi 是否已開啟
    pub async fn is_wifi_enabled(&self) -> AppResult<bool> {
        todo!()
    }

    /// 啟用或停用 Wi-Fi
    pub async fn set_wifi_enabled(&self, _enabled: bool) -> AppResult<()> {
        todo!()
    }

    /// 掃描可用的 Wi-Fi 列表
    pub async fn scan_access_points(&self) -> AppResult<Vec<AccessPointInfo>> {
        todo!()
    }

    /// 連接指定 SSID
    pub async fn connect(&self, _ssid: &str, _password: Option<&str>) -> AppResult<()> {
        todo!()
    }

    /// 斷開目前的 Wi-Fi 連線
    pub async fn disconnect(&self) -> AppResult<()> {
        todo!()
    }
}
