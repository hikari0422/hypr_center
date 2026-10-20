use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keybinding {
    pub modifiers: Vec<String>,   // 例：["SUPER", "SHIFT"]
    pub key: String,              // 例："Q", "Return"
    pub dispatcher: String,       // 例："exec", "killactive", "togglefloating"
    pub arg: String,              // 例："kitty", ""
}

pub struct KeybindingService;

impl KeybindingService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_keybindings(&self) -> AppResult<Vec<Keybinding>> {
        todo!()
    }

    pub fn save_keybindings(&self, _keybindings: &[Keybinding]) -> AppResult<()> {
        todo!()
    }
}
