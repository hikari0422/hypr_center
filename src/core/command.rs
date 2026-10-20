use crate::error::{AppError, AppResult};
use std::process::Output;
use tokio::process::Command;

pub struct PolkitCommand;

impl PolkitCommand {
    /// 透過 Polkit (pkexec) 以 root 權限執行外部指令
    /// 系統會自動喚起 hyprpolkitagent 彈出認證視窗
    pub async fn run(program: &str, args: &[&str]) -> AppResult<Output> {
        let output = Command::new("pkexec")
            .arg(program)
            .args(args)
            .output()
            .await
            .map_err(AppError::Io)?;

        // pkexec 回傳 126 代表使用者在認證視窗中取消或關閉
        if output.status.code() == Some(126) {
            return Err(AppError::AuthenticationCancelled);
        }

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::CommandFailed {
                cmd: format!("pkexec {program} {}", args.join(" ")),
                reason: stderr.trim().to_string(),
            });
        }

        Ok(output)
    }

    /// 執行提權指令並直接取得 UTF-8 stdout 文字輸出
    pub async fn run_stdout(program: &str, args: &[&str]) -> AppResult<String> {
        let output = Self::run(program, args).await?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
