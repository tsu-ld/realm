use super::types::{DependencyCheckResult, DependencyStatus, DependencyType};
use super::util::{self, get_bin_dir};
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct ProjectResponse {
    versions: Vec<String>,
}

#[derive(Deserialize)]
struct VersionResponse {
    builds: Vec<u32>,
}

const PAPERMC_URL: &str = "https://api.papermc.io/v2/projects/paper";

pub async fn check() -> Result<DependencyCheckResult> {
    let bin_dir = get_bin_dir()?;
    let jar_path = bin_dir.join("server.jar");

    if !jar_path.exists() {
        return Ok(DependencyCheckResult {
            dependency: DependencyType::ServerJar,
            status: DependencyStatus::Missing,
        });
    }

    let metadata = std::fs::metadata(&jar_path)?;
    if metadata.len() == 0 {
        return Ok(DependencyCheckResult {
            dependency: DependencyType::ServerJar,
            status: DependencyStatus::Invalid("File is empty".into()),
        });
    }

    Ok(DependencyCheckResult {
        dependency: DependencyType::ServerJar,
        status: DependencyStatus::Valid,
    })
}

pub async fn download() -> Result<()> {
    let client = util::create_client()?;
    
    let project_resp: ProjectResponse = client.get(PAPERMC_URL).send().await?.json().await?;
    let latest_version = project_resp.versions.last().ok_or_else(|| anyhow::anyhow!("No versions found"))?;

    let version_url = format!("{}/versions/{}", PAPERMC_URL, latest_version);
    let version_resp: VersionResponse = client.get(&version_url).send().await?.json().await?;
    let latest_build = version_resp.builds.last().ok_or_else(|| anyhow::anyhow!("No builds found"))?;

    let download_url = format!(
        "{}/builds/{}/downloads/paper-{}-{}.jar",
        version_url, latest_build, latest_version, latest_build
    );

    let bin_dir = get_bin_dir()?;
    let temp_path = util::download_file(&download_url, "server.jar.temp").await?;
    
    std::fs::rename(temp_path, bin_dir.join("server.jar"))?;

    Ok(())
}
