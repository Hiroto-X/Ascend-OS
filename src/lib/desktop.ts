import { invoke } from "@tauri-apps/api/core";
import type {
  SettingsResponse,
  UpdateAppearanceThemeRequest,
} from "../types/settings";

export class DesktopBridgeError extends Error {
  constructor(
    public readonly code: string,
    message: string,
  ) {
    super(message);
    this.name = "DesktopBridgeError";
  }
}

function isStructuredError(value: unknown): value is { code: string; message: string } {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const candidate = value as Record<string, unknown>;
  return typeof candidate.code === "string" && typeof candidate.message === "string";
}

async function call<T>(command: string, payload?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, payload);
  } catch (error) {
    if (isStructuredError(error)) {
      throw new DesktopBridgeError(error.code, error.message);
    }

    throw new DesktopBridgeError(
      "bridge_error",
      "Ascend could not complete that request. Please try again.",
    );
  }
}

export const desktop = {
  getSettings: () => call<SettingsResponse>("get_settings"),
  updateAppearanceTheme: (request: UpdateAppearanceThemeRequest) =>
    call<SettingsResponse>("update_appearance_theme", { request }),
};
