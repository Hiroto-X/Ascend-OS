use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    error::AppResult,
    settings::domain::{AppearanceTheme, Settings},
};

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn load_appearance_theme(&self) -> AppResult<Option<AppearanceTheme>>;
    async fn save_appearance_theme(&self, theme: AppearanceTheme) -> AppResult<()>;
}

#[derive(Clone)]
pub struct SettingsService {
    repository: Arc<dyn SettingsRepository>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn SettingsRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_settings(&self) -> AppResult<Settings> {
        Ok(Settings {
            appearance_theme: self.repository.load_appearance_theme().await?,
        })
    }

    pub async fn update_appearance_theme(&self, theme: AppearanceTheme) -> AppResult<Settings> {
        self.repository.save_appearance_theme(theme).await?;
        self.get_settings().await
    }
}
