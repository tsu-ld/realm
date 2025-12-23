use crate::services::dependencies::util::get_bin_dir;
use anyhow::{anyhow, Context, Result};
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Window};
use tokio::time::timeout;

use super::output_parser::{parse_playit_output, PlayitOutput};
use super::PlayitState;

const AUTH_TIMEOUT_SECONDS: u64 = 15;
const PLAYIT_IP_EVENT: &str = "playit-ip";
const PLAYIT_LOG_EVENT: &str = "playit-log";

pub async fn run_playit() -> Result<PlayitOutput> {
    let playit_path = get_bin_dir()?.join("playit.exe");

    if !playit_path.exists() {
        return Err(anyhow!("playit.exe not found in bin directory"));
    }

    let mut child = tokio::process::Command::new(&playit_path)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn playit process")?;

    let stdout = child.stdout.take()
        .ok_or_else(|| anyhow!("Failed to capture stdout"))?;

    let stderr = child.stderr.take()
        .ok_or_else(|| anyhow!("Failed to capture stderr"))?;

    let result = timeout(
        Duration::from_secs(AUTH_TIMEOUT_SECONDS),
        parse_playit_output(stdout, stderr),
    ).await;

    let _ = child.kill().await;

    match result {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(anyhow!("Timeout waiting for playit output")),
    }
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

fn stream_output<R: Read + Send + 'static>(stream: R, window: Window) {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = window.emit(PLAYIT_LOG_EVENT, &l);
                
                if let Some(ip) = try_extract_tunnel_address(&l) {
                    let _ = window.emit(PLAYIT_IP_EVENT, ip);
                }
            }
        }
    });
}

pub fn start(window: Window, state: &PlayitState) -> Result<()> {
    let playit_path = get_bin_dir()?.join("playit.exe");

    if !playit_path.exists() {
        return Err(anyhow!("playit.exe not found"));
    }

    let mut child = Command::new(playit_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        stream_output(stdout, window.clone());
    }
    if let Some(stderr) = child.stderr.take() {
        stream_output(stderr, window);
    }

    let mut guard = state.child.lock().unwrap();
    *guard = Some(child);

    Ok(())
}

pub fn stop(state: &PlayitState) -> Result<()> {
    let mut guard = state.child.lock().unwrap();
    if let Some(ref mut child) = *guard {
        let _ = child.kill();
        *guard = None;
    }
    Ok(())
}
