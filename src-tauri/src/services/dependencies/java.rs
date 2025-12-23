use super::types::{DependencyCheckResult, DependencyStatus, DependencyType};
use super::util::{self, get_bin_dir};
use anyhow::Result;
use std::process::Command;

const JAVA_URL: &str = "https://api.adoptium.net/v3/binary/latest/21/ga/windows/x64/jdk/hotspot/normal/eclipse?project=jdk";

fn result(status: DependencyStatus) -> DependencyCheckResult {
    DependencyCheckResult {
        dependency: DependencyType::Java,
        status,
    }
}

fn is_java_working(java_command: &str) -> bool {
    Command::new(java_command)
        .arg("-version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

pub async fn check() -> Result<DependencyCheckResult> {
    if is_java_working("java") {
        return Ok(result(DependencyStatus::Valid));
    }

    let bin_dir = get_bin_dir()?;
    let java_path = bin_dir.join("java").join("bin").join("java.exe");

    if !java_path.exists() {
        return Ok(result(DependencyStatus::Missing));
    }

    if is_java_working(java_path.to_str().unwrap_or("")) {
        return Ok(result(DependencyStatus::Valid));
    }

    Ok(result(DependencyStatus::Invalid("Java executable returned error".into())))
}

pub async fn download() -> Result<()> {
    let bin_dir = get_bin_dir()?;
    let java_dir = bin_dir.join("java");

    let extract_dir = util::download_and_extract_zip(JAVA_URL).await?;

    let mut jdk_folder: Option<std::path::PathBuf> = None;
    for entry in std::fs::read_dir(&extract_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("jdk-") {
                    jdk_folder = Some(path);
                    break;
                }
            }
        }
    }

    if let Some(jdk_path) = jdk_folder {
        std::fs::rename(&jdk_path, &java_dir)?;
    }

    if extract_dir.exists() {
        let _ = std::fs::remove_dir_all(&extract_dir);
    }

    Ok(())
}

