use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Network(String),
    Bluetooth(String),
    Dbus(String),
    HyprlandIpc(String),
    Package(String),
    Config(String),
    Io(std::io::Error),
    /// 使用者在 hyprpolkitagent 彈窗中點擊取消或關閉
    AuthenticationCancelled,
    CommandFailed { cmd: String, reason: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(msg) => write!(f, "Network error: {msg}"),
            Self::Bluetooth(msg) => write!(f, "Bluetooth error: {msg}"),
            Self::Dbus(msg) => write!(f, "D-Bus error: {msg}"),
            Self::HyprlandIpc(msg) => write!(f, "Hyprland IPC error: {msg}"),
            Self::Package(msg) => write!(f, "Package management error: {msg}"),
            Self::Config(msg) => write!(f, "Config error: {msg}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::AuthenticationCancelled => write!(f, "Authentication was cancelled by the user"),
            Self::CommandFailed { cmd, reason } => write!(f, "Command '{cmd}' failed: {reason}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
