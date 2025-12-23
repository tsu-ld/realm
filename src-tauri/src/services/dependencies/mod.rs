pub mod types;
pub mod java;
pub mod minecraft;
pub mod playit;
pub mod util;

use types::{DependencyCheckResult, DependencyType};
use anyhow::Result;

pub async fn check_deps() -> Result<Vec<DependencyCheckResult>> {
    let mut results = Vec::new();
    results.push(java::check().await?);
    results.push(minecraft::check().await?);
    results.push(playit::check().await?);
    Ok(results)
}

use tauri::Emitter;

pub async fn download_deps(window: &tauri::Window, deps: Vec<DependencyType>) -> Result<()> {
    for dep in deps {
        match dep {
            DependencyType::ServerJar => minecraft::download().await?,
            DependencyType::Playit => playit::download().await?,
            DependencyType::Java => java::download().await?,
        }
        window.emit("dep-downloaded", &dep)?;
    }
    Ok(())
}
