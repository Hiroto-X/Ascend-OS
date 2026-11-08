use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    AppState,
    error::AppResult,
    settings::domain::{AppearanceTheme, Settings},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppearanceThemeRequest {
    pub appearance_theme: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub appearance_theme: Option<&'static str>,
}

impl From<Settings> for SettingsResponse {
    fn from(settings: Settings) -> Self {
        Self {
            appearance_theme: settings.appearance_theme.map(AppearanceTheme::as_str),
        }
    }
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> AppResult<SettingsResponse> {
    state.settings.get_settings().await.map(Into::into)
}

#[tauri::command]
pub async fn update_appearance_theme(
    request: UpdateAppearanceThemeRequest,
    state: State<'_, AppState>,
) -> AppResult<SettingsResponse> {
    let theme = AppearanceTheme::parse(&request.appearance_theme)?;
    state
        .settings
        .update_appearance_theme(theme)
        .await
        .map(Into::into)
}

#[cfg(test)]
mod tests {
    use super::UpdateAppearanceThemeRequest;
    use crate::settings::domain::AppearanceTheme;

    #[test]
    fn command_input_uses_the_frontend_contract_shape() {
        let request: UpdateAppearanceThemeRequest =
            serde_json::from_str(r#"{"appearanceTheme":"system"}"#).expect("request deserializes");

        assert_eq!(
            AppearanceTheme::parse(&request.appearance_theme)
                .expect("theme validates")
                .as_str(),
            "system"
        );
    }
}
