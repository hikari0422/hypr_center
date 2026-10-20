use crate::error::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BatteryInfo {
    pub percentage: f32,
    pub is_charging: bool,
    pub health_percentage: Option<f32>,
}

pub struct PowerService;

impl PowerService {
    pub async fn new() -> AppResult<Self> {
        Ok(Self)
    }

    /// 取得電池與電源狀態（若為桌機則可能無電池資訊）
    pub async fn get_battery_info(&self) -> AppResult<Option<BatteryInfo>> {
        todo!()
    }

    /// 取得目前電源設定檔
    pub async fn get_power_profile(&self) -> AppResult<PowerProfile> {
        todo!()
    }

    /// 設定電源設定檔
    pub async fn set_power_profile(&self, _profile: PowerProfile) -> AppResult<()> {
        todo!()
    }

    /// 睡眠
    pub async fn suspend(&self) -> AppResult<()> {
        todo!()
    }

    /// 重新啟動
    pub async fn reboot(&self) -> AppResult<()> {
        todo!()
    }

    /// 關機
    pub async fn power_off(&self) -> AppResult<()> {
        todo!()
    }
}
