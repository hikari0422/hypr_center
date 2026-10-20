use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupService {
    pub command: String,          // 例："waybar", "hyprpaper"
    pub description: Option<String>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentVariable {
    pub key: String,              // 例："XCURSOR_SIZE"
    pub value: String,            // 例："24"
}

pub struct AutostartService;

impl AutostartService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_startup_services(&self) -> AppResult<Vec<StartupService>> {
        todo!()
    }

    pub fn save_startup_services(&self, _services: &[StartupService]) -> AppResult<()> {
        todo!()
    }

    pub fn get_environment_variables(&self) -> AppResult<Vec<EnvironmentVariable>> {
        todo!()
    }

    pub fn save_environment_variables(&self, _envs: &[EnvironmentVariable]) -> AppResult<()> {
        todo!()
    }
}
