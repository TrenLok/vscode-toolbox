import type { AppSettings } from '~/types/app-settings';

export function useAppSettings() {
  const appSettingsStore = useAppSettingsStore();

  const vsCodeSync = computed(() => appSettingsStore.settings.vsCodeSync);
  const autoCheckUpdates = computed(() => appSettingsStore.settings.autoCheckUpdates);

  const { load, save, clear } = useTauriStore<AppSettings>({
    file: 'settings.json',
    key: 'settings',
    defaultValue: { ...appDefaultSettings },
    logPrefix: 'App Settings',
    latestVersion: 1,
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

  return {
    vsCodeSync,
    autoCheckUpdates,
    clearDb,
    saveToDb,
    loadFromDb,
    switchVsCodeSync,
    switchAutoCheckUpdates,
  };
}
