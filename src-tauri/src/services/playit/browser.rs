use anyhow::{Context, Result};

pub fn open_claim_url(url: &str) -> Result<()> {
    open::that(url).context("Failed to open browser for playit claim URL")
}
