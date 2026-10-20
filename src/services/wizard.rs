use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WizardStep {
    PluginSelection,
    KeybindingSetup,
    Tutorial,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WizardPluginItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_selected: bool,
    pub is_installed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendedKeybinding {
    pub action_name: String,
    pub key_combination: String,
    pub description: String,
}

pub struct WizardService;

impl WizardService {
    pub fn new() -> Self {
        Self
    }

    /// 取得初次安裝推薦的插件清單
    pub fn get_recommended_plugins(&self) -> AppResult<Vec<WizardPluginItem>> {
        todo!()
    }

    /// 套用使用者勾選的插件安裝
    pub async fn install_selected_plugins(&self, _plugin_ids: &[String]) -> AppResult<()> {
        todo!()
    }

    /// 取得推薦的常用快捷鍵預設值
    pub fn get_preset_keybindings(&self) -> AppResult<Vec<RecommendedKeybinding>> {
        todo!()
    }

    /// 套用設定的快捷鍵
    pub fn apply_preset_keybindings(&self, _keybindings: &[RecommendedKeybinding]) -> AppResult<()> {
        todo!()
    }

    /// 完成初次引導安裝
    pub fn complete_wizard(&self) -> AppResult<()> {
        todo!()
    }
}
