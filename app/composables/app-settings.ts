import type { AppSettings, AppTheme, ProjectIconStyle } from '~/types/app-settings';
import { invoke } from '@tauri-apps/api/core';

function shouldRelaunchForThemeSwitch(currentTheme: AppTheme, nextTheme: AppTheme) {
  return !import.meta.dev
    && useTauriOsPlatform() === 'macos'
    && currentTheme !== nextTheme;
}

export function useAppSettings() {
  const appSettingsStore = useAppSettingsStore();

  const vsCodeSync = computed(() => appSettingsStore.settings.vsCodeSync);
  const autoCheckUpdates = computed(() => appSettingsStore.settings.autoCheckUpdates);
  const theme = computed(() => appSettingsStore.settings.theme);
  const projectIconStyle = computed(() => appSettingsStore.settings.projectIconStyle);
  const revealShortcut = computed(() => appSettingsStore.settings.revealShortcut);

  const { load, save, clear } = useTauriStore<AppSettings>({
    file: 'settings.json',
    key: 'settings',
    defaultValue: { ...appDefaultSettings },
    logPrefix: 'App Settings',
    latestVersion: 4,
    migrations: {
      1: (data: Partial<Omit<AppSettings, 'theme'>>): AppSettings => ({
        ...appDefaultSettings,
        ...data,
        theme: 'default',
      }),
      2: (data: Partial<Omit<AppSettings, 'projectIconStyle'>>): AppSettings => ({
        ...appDefaultSettings,
        ...data,
        projectIconStyle: 'default',
      }),
      3: (data: Partial<Omit<AppSettings, 'revealShortcut'>>): AppSettings => ({
        ...appDefaultSettings,
        ...data,
        revealShortcut: null,
      }),
    },
  });

  async function saveToDb() {
    await save(appSettingsStore.settings);
  }

  async function loadFromDb() {
    const settings = await load();
    appSettingsStore.settings = settings;

    await setRevealShortcutToBackend(settings.revealShortcut);
  }

  async function clearDb() {
    appSettingsStore.settings = { ...appDefaultSettings };
    await clear();
    await setRevealShortcut(null);
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
    const currentTheme = appSettingsStore.settings.theme;
    const shouldRelaunch = shouldRelaunchForThemeSwitch(currentTheme, value);

    appSettingsStore.settings.theme = value;

    if (shouldRelaunch) {
      await saveToDb();

      try {
        await useTauriProcessRelaunch();
        return;
      } catch (error_) {
        useTauriLogError(`App Settings relaunch after theme switch error: ${error_}`);
      }
    }

    applyThemeClass(value);
    await saveToDb();

    try {
      await invoke('set_window_theme', { theme: value });
    } catch (error_) {
      useTauriLogError(`App Settings theme switch error: ${error_}`);
    }
  }

  async function switchProjectIconStyle(value: ProjectIconStyle) {
    appSettingsStore.settings.projectIconStyle = value;
    await saveToDb();
  }

  async function setRevealShortcutToBackend(value: string | null) {
    await invoke('set_reveal_shortcut', { shortcut: value });
  }

  async function setRevealShortcut(value: string | null) {
    await setRevealShortcutToBackend(value);
    appSettingsStore.settings.revealShortcut = value;
    await saveToDb();
  }

  return {
    vsCodeSync,
    autoCheckUpdates,
    theme,
    projectIconStyle,
    revealShortcut,
    clearDb,
    saveToDb,
    loadFromDb,
    switchVsCodeSync,
    switchAutoCheckUpdates,
    switchTheme,
    switchProjectIconStyle,
    setRevealShortcut,
  };
}
