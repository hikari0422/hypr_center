use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayResolution {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayOutput {
    pub id: String,                 // 介面名稱，例："eDP-1", "DP-1", "HDMI-A-1"
    pub description: String,        // 廠牌型號，例："LG Electronics 27GL850"
    pub resolution: DisplayResolution,
    pub scale: f32,                 // 縮放倍率，例：1.0, 1.25, 2.0
    pub x: i32,                     // 桌面畫布相對座標 X
    pub y: i32,                     // 桌面畫布相對座標 Y
    pub rotation: u16,              // 旋轉角度 (0, 90, 180, 270)
    pub is_enabled: bool,
    pub is_primary: bool,
    pub available_modes: Vec<DisplayResolution>,
}

pub struct DisplayService;

impl DisplayService {
    pub async fn new() -> AppResult<Self> {
        Ok(Self)
    }

    /// 取得目前連接的所有顯示器輸出狀態與支援的顯示模式
    pub async fn get_displays(&self) -> AppResult<Vec<DisplayOutput>> {
        todo!()
    }

    /// 套用單一或多螢幕配置（解析度、排列位置、更新率、縮放比例）
    pub async fn apply_display_config(&self, _display: &DisplayOutput) -> AppResult<()> {
        todo!()
    }

    /// 設定螢幕鏡像輸出 (Mirror)
    pub async fn set_mirror(&self, _source_id: &str, _target_id: &str) -> AppResult<()> {
        todo!()
    }
}
