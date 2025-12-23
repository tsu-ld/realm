use super::types::{DependencyCheckResult, DependencyStatus, DependencyType};
use super::util::{self, get_bin_dir};
use anyhow::Result;
use std::process::Command;

const PLAYIT_URL: &str = "https://github.com/playit-cloud/playit-agent/releases/download/v0.17.0-rc2/playit-windows-x86_64.exe";

pub async fn check() -> Result<DependencyCheckResult> {
    let bin_dir = get_bin_dir()?;
    let playit_path = bin_dir.join("playit.exe");

    if !playit_path.exists() {
        return Ok(DependencyCheckResult {
            dependency: DependencyType::Playit,
            status: DependencyStatus::Missing,
        });
    }

    let output = Command::new(&playit_path).arg("--version").output();

    match output {
        Ok(out) if out.status.success() => Ok(DependencyCheckResult {
            dependency: DependencyType::Playit,
            status: DependencyStatus::Valid,
        }),
        Ok(_) => Ok(DependencyCheckResult {
            dependency: DependencyType::Playit,
            status: DependencyStatus::Invalid("Playit executable returned error".into()),
        }),
        Err(e) => Ok(DependencyCheckResult {
            dependency: DependencyType::Playit,
            status: DependencyStatus::Invalid(e.to_string()),
        }),
    }
}

pub async fn download() -> Result<()> {
    let bin_dir = get_bin_dir()?;
    let temp_path = util::download_file(PLAYIT_URL, "playit.exe.temp").await?;

    std::fs::rename(temp_path, bin_dir.join("playit.exe"))?;

    Ok(())
}
