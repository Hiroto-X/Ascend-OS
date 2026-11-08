use std::{path::Path, str::FromStr, time::Duration};

use async_trait::async_trait;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

use crate::{
    error::{AppError, AppResult},
    settings::{application::SettingsRepository, domain::AppearanceTheme},
};

const APPEARANCE_THEME_KEY: &str = "appearance.theme";

pub struct SqliteSettingsRepository {
    pub(crate) pool: SqlitePool,
}

impl SqliteSettingsRepository {
    pub async fn connect(database_path: &Path) -> AppResult<Self> {
        let options = SqliteConnectOptions::from_str(database_path.to_string_lossy().as_ref())
            .map_err(|_| AppError::Storage)?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(5));

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|_| AppError::Storage)?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|_| AppError::Storage)?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl SettingsRepository for SqliteSettingsRepository {
    async fn load_appearance_theme(&self) -> AppResult<Option<AppearanceTheme>> {
        let value = sqlx::query_scalar::<_, String>(
            "SELECT setting_value FROM settings WHERE setting_key = ?",
        )
        .bind(APPEARANCE_THEME_KEY)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AppError::Storage)?;

        value
            .map(|stored| AppearanceTheme::parse(&stored).map_err(|_| AppError::Storage))
            .transpose()
    }

    async fn save_appearance_theme(&self, theme: AppearanceTheme) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO settings (setting_key, setting_value) VALUES (?, ?) \
             ON CONFLICT(setting_key) DO UPDATE SET setting_value = excluded.setting_value, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
        )
        .bind(APPEARANCE_THEME_KEY)
        .bind(theme.as_str())
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::Storage)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sqlx::Row;

    use super::SqliteSettingsRepository;
    use crate::{
        error::AppError,
        settings::{application::SettingsService, domain::AppearanceTheme},
    };

    #[test]
    fn migrations_create_the_settings_schema() {
        tauri::async_runtime::block_on(async {
            let temporary_directory = tempfile::tempdir().expect("temporary test directory");
            let database_path = temporary_directory.path().join("ascend.sqlite");
            let repository = SqliteSettingsRepository::connect(&database_path)
                .await
                .expect("database initializes");

            let migration = sqlx::query("SELECT version FROM _sqlx_migrations")
                .fetch_one(&repository.pool)
                .await
                .expect("migration is recorded");

            assert_eq!(migration.get::<i64, _>("version"), 1);
        });
    }

    #[test]
    fn theme_persists_across_database_restarts() {
        tauri::async_runtime::block_on(async {
            let temporary_directory = tempfile::tempdir().expect("temporary test directory");
            let database_path = temporary_directory.path().join("ascend.sqlite");
            let first_repository = Arc::new(
                SqliteSettingsRepository::connect(&database_path)
                    .await
                    .expect("database initializes"),
            );
            let first_service = SettingsService::new(first_repository.clone());
            first_service
                .update_appearance_theme(AppearanceTheme::Dark)
                .await
                .expect("theme saves");
            drop(first_service);
            drop(first_repository);

            let second_repository = Arc::new(
                SqliteSettingsRepository::connect(&database_path)
                    .await
                    .expect("database reopens"),
            );
            let second_service = SettingsService::new(second_repository);

            assert_eq!(
                second_service
                    .get_settings()
                    .await
                    .expect("settings load")
                    .appearance_theme,
                Some(AppearanceTheme::Dark)
            );
        });
    }

    #[test]
    fn unsupported_themes_are_rejected() {
        assert!(matches!(
            AppearanceTheme::parse("ultraviolet"),
            Err(AppError::Validation)
        ));
    }
}
