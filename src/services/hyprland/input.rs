use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq)]
pub struct InputGestureConfig {
    pub kb_layout: String,        // 鍵盤佈局，例："us", "tw"
    pub kb_variant: String,
    pub sensitivity: f32,         // 滑鼠靈敏度 (-1.0 ~ 1.0)
    pub natural_scroll: bool,     // 自然捲動
    pub tap_to_click: bool,       // 輕觸點擊
    pub workspace_swipe: bool,    // 觸控板工作區滑動手勢
    pub workspace_swipe_fingers: u32, // 手勢指數
}

pub struct InputService;

impl InputService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_input_gesture_config(&self) -> AppResult<InputGestureConfig> {
        todo!()
    }

    pub fn save_input_gesture_config(&self, _config: &InputGestureConfig) -> AppResult<()> {
        todo!()
    }
}
