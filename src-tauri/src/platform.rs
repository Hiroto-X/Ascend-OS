use std::{
    fs,
    path::{Path, PathBuf},
};

use tauri::{AppHandle, Manager};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::{AppError, AppResult};

pub struct ApplicationPaths {
    pub data_directory: PathBuf,
    pub logs_directory: PathBuf,
}

impl ApplicationPaths {
    pub fn resolve(app: &AppHandle) -> AppResult<Self> {
        let root = app
            .path()
            .app_data_dir()
            .map_err(|_| AppError::DataDirectory)?;
        let environment = if cfg!(debug_assertions) {
            "development"
        } else {
            "production"
        };
        let environment_root = root.join(environment);

        Ok(Self {
            data_directory: environment_root.join("data"),
            logs_directory: environment_root.join("logs"),
        })
    }
}

pub fn ensure_directory(directory: &Path) -> AppResult<()> {
    fs::create_dir_all(directory).map_err(|_| AppError::DataDirectory)
}

pub fn initialize_logging(logs_directory: &Path) -> AppResult<WorkerGuard> {
    ensure_directory(logs_directory)?;
    let appender = tracing_appender::rolling::daily(logs_directory, "ascend-os.jsonl");
    let (writer, guard) = tracing_appender::non_blocking(appender);

    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ascend_os_lib=info")),
        )
        .with(tracing_subscriber::fmt::layer().json().with_writer(writer));

    // A test host or embedding application may already own the global subscriber.
    // In that case Ascend remains operational without replacing that host's logging policy.
    let _ = subscriber.try_init();
    Ok(guard)
}
