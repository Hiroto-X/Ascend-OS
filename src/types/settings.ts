export const appearanceThemes = ["light", "dark", "system"] as const;

export type AppearanceTheme = (typeof appearanceThemes)[number];

export interface SettingsResponse {
  appearanceTheme: AppearanceTheme | null;
}

export interface UpdateAppearanceThemeRequest {
  appearanceTheme: AppearanceTheme;
}
