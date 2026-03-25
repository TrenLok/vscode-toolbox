import type { AppSettings, AppTheme } from '~/types/app-settings';
import { invoke } from '@tauri-apps/api/core';

export function useAppSettings() {
  const appSettingsStore = useAppSettingsStore();

  const vsCodeSync = computed(() => appSettingsStore.settings.vsCodeSync);
  const autoCheckUpdates = computed(() => appSettingsStore.settings.autoCheckUpdates);
  const theme = computed(() => appSettingsStore.settings.theme);

  const { load, save, clear } = useTauriStore<AppSettings>({
    file: 'settings.json',
    key: 'settings',
    defaultValue: { ...appDefaultSettings },
    logPrefix: 'App Settings',
    latestVersion: 2,
    migrations: {
      1: (data: Partial<Omit<AppSettings, 'theme'>>): AppSettings => ({
        ...appDefaultSettings,
        ...data,
        theme: 'default',
      }),
    },
  });

  async function saveToDb() {
    await save(appSettingsStore.settings);
  }

  async function loadFromDb() {
    const settings = await load();
    appSettingsStore.settings = settings;
  }

  async function clearDb() {
    appSettingsStore.settings = { ...appDefaultSettings };
    await clear();
  }

  async function switchVsCodeSync() {
    appSettingsStore.settings.vsCodeSync = !appSettingsStore.settings.vsCodeSync;
    await saveToDb();
  }

  async function switchAutoCheckUpdates() {
    appSettingsStore.settings.autoCheckUpdates = !appSettingsStore.settings.autoCheckUpdates;
    await saveToDb();
  }

  async function switchTheme(value: AppTheme) {
    appSettingsStore.settings.theme = value;
    await saveToDb();

    try {
      await invoke('set_window_theme', { theme: value });
    } catch (error_) {
      useTauriLogError(`App Settings theme switch error: ${error_}`);
    }
  }

  return {
    vsCodeSync,
    autoCheckUpdates,
    theme,
    clearDb,
    saveToDb,
    loadFromDb,
    switchVsCodeSync,
    switchAutoCheckUpdates,
    switchTheme,
  };
}
