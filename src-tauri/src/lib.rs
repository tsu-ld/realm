pub mod commands;
pub mod services;

use services::server::ServerState;
use services::playit::PlayitState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ServerState::default())
        .manage(PlayitState::default())
        .invoke_handler(tauri::generate_handler![
            commands::check_deps,
            commands::download_deps,
            commands::configure_playit,
            commands::start_playit,
            commands::start_server,
            commands::stop_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
