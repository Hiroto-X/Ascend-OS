mod error;
mod platform;
mod settings;

use std::sync::Arc;

use platform::{ApplicationPaths, ensure_directory, initialize_logging};
use settings::{application::SettingsService, infrastructure::SqliteSettingsRepository};
use tauri::Manager;
use tracing_appender::non_blocking::WorkerGuard;

pub struct AppState {
    settings: SettingsService,
    // The guard keeps the non-blocking, JSON logging worker alive for the app lifetime.
    _logging_guard: WorkerGuard,
}

impl AppState {
    async fn initialize(app: &tauri::AppHandle) -> error::AppResult<Self> {
        let paths = ApplicationPaths::resolve(app)?;
        ensure_directory(&paths.data_directory)?;
        let logging_guard = initialize_logging(&paths.logs_directory)?;
        let database_path = paths.data_directory.join("ascend.sqlite");
        let repository = Arc::new(SqliteSettingsRepository::connect(&database_path).await?);

        tracing::info!(
            event = "settings_database_initialized",
            "settings database initialized"
        );

        Ok(Self {
            settings: SettingsService::new(repository),
            _logging_guard: logging_guard,
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let outcome = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = tauri::async_runtime::block_on(AppState::initialize(app.handle()))?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            settings::commands::get_settings,
            settings::commands::update_appearance_theme
        ])
        .run(tauri::generate_context!());

    if outcome.is_err() {
        eprintln!("Ascend OS could not start.");
    }
}
