use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub is_first_run_completed: bool,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            is_first_run_completed: false,
            theme: "dark".to_string(),
        }
    }
}

pub struct ConfigService;

impl ConfigService {
    pub fn new() -> Self {
        Self
    }

    /// 讀取應用程式設定
    pub fn load_config(&self) -> AppResult<AppConfig> {
        todo!()
    }

    /// 儲存應用程式設定
    pub fn save_config(&self, _config: &AppConfig) -> AppResult<()> {
        todo!()
    }

    /// 檢查是否為初次安裝/啟動
    pub fn is_first_run(&self) -> AppResult<bool> {
        todo!()
    }

    /// 標記初次導引安裝已完成
    pub fn mark_first_run_completed(&self) -> AppResult<()> {
        todo!()
    }
}
