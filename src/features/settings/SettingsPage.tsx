import { useEffect } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { desktop } from "../../lib/desktop";
import {
  appearanceThemes,
  type AppearanceTheme,
  type SettingsResponse,
} from "../../types/settings";

const appearanceThemeLabels: Record<AppearanceTheme, string> = {
  light: "Light",
  dark: "Dark",
  system: "Use system setting",
};

function isAppearanceTheme(value: string): value is AppearanceTheme {
  return appearanceThemes.some((theme) => theme === value);
}

export function SettingsPage() {
  const queryClient = useQueryClient();
  const settingsQuery = useQuery({
    queryKey: ["settings"],
    queryFn: desktop.getSettings,
  });
  const updateTheme = useMutation({
    mutationFn: desktop.updateAppearanceTheme,
    onSuccess: (settings) => {
      queryClient.setQueryData(["settings"], settings);
    },
  });

  const appearanceTheme = settingsQuery.data?.appearanceTheme;
  useEffect(() => {
    if (appearanceTheme === null || appearanceTheme === undefined) {
      delete document.documentElement.dataset.appearance;
      return;
    }

    document.documentElement.dataset.appearance = appearanceTheme;
  }, [appearanceTheme]);

  if (settingsQuery.isPending) {
    return <p role="status">Loading settings…</p>;
  }

  if (settingsQuery.isError) {
    return (
      <section className="settings-page" aria-labelledby="settings-heading">
        <h1 id="settings-heading">Settings</h1>
        <p role="alert">We could not load your settings.</p>
        <button type="button" onClick={() => settingsQuery.refetch()}>
          Try again
        </button>
      </section>
    );
  }

  const settings: SettingsResponse = settingsQuery.data;
  const selectedTheme = settings.appearanceTheme ?? "";
  const updateError = updateTheme.error;

  return (
    <section className="settings-page" aria-labelledby="settings-heading">
      <p className="eyebrow">System</p>
      <h1 id="settings-heading">Settings</h1>
      <p>
        Choose how Ascend OS should appear. This preference saves immediately on
        this device.
      </p>

      {settings.appearanceTheme === null ? (
        <p className="first-run-notice" role="status">
          Choose an appearance preference to finish setting up Settings.
        </p>
      ) : null}

      <div className="setting-row">
        <div>
          <label htmlFor="appearance-theme">Appearance</label>
          <p id="appearance-theme-description">
            Select a light, dark, or system appearance.
          </p>
        </div>
        <select
          id="appearance-theme"
          value={selectedTheme}
          aria-describedby="appearance-theme-description"
          disabled={updateTheme.isPending}
          onChange={(event) => {
            const nextTheme = event.currentTarget.value;
            if (isAppearanceTheme(nextTheme)) {
              updateTheme.mutate({ appearanceTheme: nextTheme });
            }
          }}
        >
          <option value="" disabled>
            Choose an appearance
          </option>
          {appearanceThemes.map((theme) => (
            <option key={theme} value={theme}>
              {appearanceThemeLabels[theme]}
            </option>
          ))}
        </select>
      </div>

      {updateTheme.isPending ? <p role="status">Saving appearance…</p> : null}
      {updateTheme.isSuccess ? <p role="status">Appearance saved.</p> : null}
      {updateError ? <p role="alert">We could not save your appearance.</p> : null}
    </section>
  );
}
