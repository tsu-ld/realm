use crate::services::dependencies::{self, types::{DependencyCheckResult, DependencyType}};
use crate::services::playit::{self, ConfigurePlayitResult, PlayitOutput, PlayitState};
use crate::services::server::{self, ServerState};

#[tauri::command]
pub async fn start_server(window: tauri::Window, state: tauri::State<'_, ServerState>) -> Result<(), String> {
    server::start(window, &state).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_playit(window: tauri::Window, state: tauri::State<'_, PlayitState>) -> Result<(), String> {
    playit::start(window, &state).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_server(
    server_state: tauri::State<'_, ServerState>,
    playit_state: tauri::State<'_, PlayitState>
) -> Result<(), String> {
    let _ = server::stop(&server_state);
    let _ = playit::stop(&playit_state);
    Ok(())
}

#[tauri::command]
pub async fn check_deps() -> Result<Vec<DependencyCheckResult>, String> {
    dependencies::check_deps()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_deps(window: tauri::Window, deps: Vec<DependencyType>) -> Result<(), String> {
    dependencies::download_deps(&window, deps)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn configure_playit() -> Result<ConfigurePlayitResult, String> {
    match playit::run_playit().await {
        Ok(PlayitOutput::TunnelAddress(tunnel_address)) => {
            Ok(ConfigurePlayitResult::AlreadyConfigured { tunnel_address })
        }
        Ok(PlayitOutput::ClaimUrl(claim_url)) => {
            if let Err(e) = playit::open_claim_url(&claim_url) {
                return Ok(ConfigurePlayitResult::Error(e.to_string()));
            }
            Ok(ConfigurePlayitResult::NeedsAuth { claim_url })
        }
        Err(e) => Ok(ConfigurePlayitResult::Error(e.to_string())),
    }
}
