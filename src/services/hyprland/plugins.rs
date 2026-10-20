use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyprlandPluginInfo {
    pub name: String,
    pub repo_url: String,
    pub is_enabled: bool,
    pub has_update: bool,
}

pub struct PluginService;

impl PluginService {
    pub fn new() -> Self {
        Self
    }

    pub async fn list_plugins(&self) -> AppResult<Vec<HyprlandPluginInfo>> {
        todo!()
    }

    pub async fn toggle_plugin(&self, _name: &str, _enabled: bool) -> AppResult<()> {
        todo!()
    }
}
