use crate::error::AppResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    PacmanRepo(String), // 如 "core", "extra"
    Aur,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageItem {
    pub name: String,
    pub version: String,
    pub description: String,
    pub is_installed: bool,
    pub source: PackageSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageProgressEvent {
    Downloading { pkg: String, percent: u32 },
    Installing { pkg: String, percent: u32 },
    Completed,
    Failed(String),
}

pub struct PackageManagerService;

impl PackageManagerService {
    pub fn new() -> Self {
        Self
    }

    /// 檢查系統可更新的套件清單
    pub async fn check_updates(&self) -> AppResult<Vec<PackageItem>> {
        todo!()
    }

    /// 搜尋官方 Repo 與 AUR 套件
    pub async fn search_packages(&self, _query: &str) -> AppResult<Vec<PackageItem>> {
        todo!()
    }

    /// 列出已安裝套件
    pub fn list_installed_packages(&self) -> AppResult<Vec<PackageItem>> {
        todo!()
    }

    /// 安裝指定套件
    pub async fn install_packages(
        &self,
        _package_names: &[String],
        _on_progress: Box<dyn Fn(PackageProgressEvent) + Send + Sync>,
    ) -> AppResult<()> {
        todo!()
    }

    /// 移除指定套件
    pub async fn remove_packages(
        &self,
        _package_names: &[String],
        _on_progress: Box<dyn Fn(PackageProgressEvent) + Send + Sync>,
    ) -> AppResult<()> {
        todo!()
    }
}
