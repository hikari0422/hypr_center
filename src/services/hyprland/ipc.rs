use crate::error::AppResult;

pub struct HyprlandIpc;

impl HyprlandIpc {
    pub fn new() -> Self {
        Self
    }

    /// 重新載入 Hyprland 設定 (hyprctl reload)
    pub async fn reload(&self) -> AppResult<()> {
        todo!()
    }
}
