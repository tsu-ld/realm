use super::dependencies::util::get_bin_dir;
use anyhow::Result;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use tauri::{Emitter, Window};

pub struct ServerState {
    pub child: Mutex<Option<Child>>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }
}

fn ensure_eula_accepted(bin_dir: &Path) -> Result<()> {
    let eula_path = bin_dir.join("eula.txt");
    if !eula_path.exists() || !std::fs::read_to_string(&eula_path)?.contains("eula=true") {
        std::fs::write(eula_path, "eula=true")?;
    }
    Ok(())
}

fn get_java_path(bin_dir: &Path) -> String {
    let java_path = bin_dir.join("java").join("bin").join("java.exe");
    if java_path.exists() {
        java_path.to_string_lossy().to_string()
    } else {
        "java".to_string()
    }
}

fn spawn_process(bin_dir: &Path, java_cmd: &str) -> Result<Child> {
    Command::new(java_cmd)
        .arg("-jar")
        .arg(bin_dir.join("server.jar"))
        .arg("nogui")
        .current_dir(bin_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(Into::into)
}

fn stream_output<R: Read + Send + 'static>(stream: R, window: Window) {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = window.emit("server-log", &l);
            }
        }
    });
}

pub fn start(window: Window, state: &ServerState) -> Result<()> {
    let bin_dir = get_bin_dir()?;
    ensure_eula_accepted(&bin_dir)?;
    
    let java_cmd = get_java_path(&bin_dir);
    let mut child = spawn_process(&bin_dir, &java_cmd)?;

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

pub fn stop(state: &ServerState) -> Result<()> {
    let mut guard = state.child.lock().unwrap();
    if let Some(ref mut child) = *guard {
        child.kill()?;
        *guard = None;
    }
    Ok(())
}
