use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
    pub layout_name: String,      // "dwindle" 或 "master"
    pub gaps_in: u32,             // 視窗內邊距
    pub gaps_out: u32,            // 視窗外邊距 (螢幕邊緣)
    pub border_size: u32,         // 邊框粗細
    pub active_border_color: String,
    pub inactive_border_color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecorationConfig {
    pub rounding: u32,            // 圓角半徑
    pub active_opacity: f32,      // 作用中視窗透明度 (0.0 ~ 1.0)
    pub inactive_opacity: f32,    // 非作用中視窗透明度
    pub blur_enabled: bool,       // 是否啟用高斯模糊
    pub blur_size: u32,           // 模糊範圍
    pub blur_passes: u32,         // 模糊運算次數
    pub shadow_enabled: bool,     // 是否啟用陰影
    pub shadow_range: u32,        // 陰影範圍
    pub shadow_color: String,     // 陰影顏色
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub bezier_curve: String,     // 自訂貝茲曲線
    pub speed: f32,
}

pub struct AppearanceService;

impl AppearanceService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_layout_config(&self) -> AppResult<LayoutConfig> {
        todo!()
    }

    pub fn save_layout_config(&self, _config: &LayoutConfig) -> AppResult<()> {
        todo!()
    }

    pub fn get_decoration_config(&self) -> AppResult<DecorationConfig> {
        todo!()
    }

    pub fn save_decoration_config(&self, _config: &DecorationConfig) -> AppResult<()> {
        todo!()
    }

    pub fn get_animations(&self) -> AppResult<AnimationConfig> {
        todo!()
    }

    pub fn save_animations(&self, _config: &AnimationConfig) -> AppResult<()> {
        todo!()
    }
}
