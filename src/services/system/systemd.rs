use crate::error::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitScope {
    System, // 系統級服務 (需要管理員權限)
    User,   // 使用者會話級服務 (systemctl --user)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveState {
    Active,       // 正在執行 (running)
    Inactive,     // 已停止 (dead)
    Failed,       // 執行崩潰/失敗 (failed)
    Activating,   // 正在啟動
    Deactivating, // 正在停止
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnableState {
    Enabled,  // 開機自啟
    Disabled, // 開機不自啟
    Masked,   // 已遮蔽 (完全無法啟動)
    Static,   // 無法自啟 (被其他服務依賴觸發)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnitInfo {
    pub name: String,
    pub description: String,
    pub active_state: ActiveState,
    pub enable_state: EnableState,
    pub scope: UnitScope,
}

pub struct SystemdService;

impl SystemdService {
    pub fn new() -> Self {
        Self
    }

    /// 列出指定範圍（系統或使用者）下的關鍵服務狀態
    pub async fn list_services(&self, _scope: UnitScope) -> AppResult<Vec<SystemdUnitInfo>> {
        todo!()
    }

    /// 啟動服務 (start)
    pub async fn start_service(&self, _unit_name: &str, _scope: UnitScope) -> AppResult<()> {
        todo!()
    }

    /// 停止服務 (stop)
    pub async fn stop_service(&self, _unit_name: &str, _scope: UnitScope) -> AppResult<()> {
        todo!()
    }

    /// 重新啟動服務 (restart)
    pub async fn restart_service(&self, _unit_name: &str, _scope: UnitScope) -> AppResult<()> {
        todo!()
    }

    /// 設定開機自動啟動狀態 (enable / disable)
    pub async fn set_service_enabled(&self, _unit_name: &str, _enabled: bool, _scope: UnitScope) -> AppResult<()> {
        todo!()
    }
}
