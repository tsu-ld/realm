use anyhow::Result;
use futures_util::StreamExt;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn get_bin_dir() -> Result<PathBuf> {
    let path = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find local data directory"))?
        .join("server_files");

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn create_client() -> Result<reqwest::Client> {
    let client = reqwest::Client::builder()
        .user_agent("realm")
        .build()?;
    Ok(client)
}

pub async fn download_file(url: &str, file_name: &str) -> Result<PathBuf> {
    let target_path = get_bin_dir()?.join(file_name);
    let client = create_client()?;
    let response = client.get(url).send().await?.error_for_status()?;
    let mut content = response.bytes_stream();
    let mut file = File::create(&target_path)?;

    while let Some(item) = content.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
    }

    Ok(target_path)
}

pub async fn download_and_extract_zip(url: &str) -> Result<PathBuf> {
    let bin_dir = get_bin_dir()?;
    let zip_path = bin_dir.join("temp.zip");
    let extract_dir = bin_dir.join("temp_extract");

    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir)?;
    }
    fs::create_dir_all(&extract_dir)?;

    download_file(url, "temp.zip").await?;

    {
        let file = File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(&extract_dir)?;
    }

    fs::remove_file(&zip_path)?;

    Ok(extract_dir)
}
