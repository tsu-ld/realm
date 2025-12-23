use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{ChildStderr, ChildStdout};

pub enum PlayitOutput {
    TunnelAddress(String),
    ClaimUrl(String),
}

pub async fn parse_playit_output(stdout: ChildStdout, stderr: ChildStderr) -> Result<PlayitOutput> {
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            line = stdout_reader.next_line() => {
                if let Some(line) = line? {
                    if let Some(output) = try_parse_line(&line) {
                        return Ok(output);
                    }
                }
            }
            line = stderr_reader.next_line() => {
                if let Some(line) = line? {
                    if let Some(output) = try_parse_line(&line) {
                        return Ok(output);
                    }
                }
            }
        }
    }
}

fn try_parse_line(line: &str) -> Option<PlayitOutput> {
    if let Some(address) = try_extract_tunnel_address(line) {
        return Some(PlayitOutput::TunnelAddress(address));
    }

    if let Some(url) = try_extract_claim_url(line) {
        return Some(PlayitOutput::ClaimUrl(url));
    }

    None
}

fn try_extract_tunnel_address(line: &str) -> Option<String> {
    if !line.contains("=>") || !line.contains("127.0.0.1") {
        return None;
    }

    line.split("=>")
        .next()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn try_extract_claim_url(line: &str) -> Option<String> {
    line.split_whitespace()
        .find(|word| word.contains("https://playit.gg/claim/"))
        .map(|url| url.to_string())
}
