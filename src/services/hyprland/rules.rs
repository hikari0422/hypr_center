use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRule {
    pub rule: String,             // 例："float", "size 800 600", "opacity 0.9"
    pub window_class: Option<String>,
    pub window_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRule {
    pub workspace_id: i32,
    pub assigned_monitor: Option<String>,
    pub is_default: bool,
    pub is_persistent: bool,
}

pub struct RuleService;

impl RuleService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_window_rules(&self) -> AppResult<Vec<WindowRule>> {
        todo!()
    }

    pub fn save_window_rules(&self, _rules: &[WindowRule]) -> AppResult<()> {
        todo!()
    }

    pub fn get_workspace_rules(&self) -> AppResult<Vec<WorkspaceRule>> {
        todo!()
    }

    pub fn save_workspace_rules(&self, _rules: &[WorkspaceRule]) -> AppResult<()> {
        todo!()
    }
}
