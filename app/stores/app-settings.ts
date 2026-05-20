import type { AppSettings } from '~/types/app-settings';

export const appDefaultSettings: AppSettings = {
  vsCodeSync: false,
  autoCheckUpdates: false,
  theme: 'default',
  projectIconStyle: 'default',
  revealShortcut: null,
};

export const useAppSettingsStore = defineStore('AppSettings', () => {
  const settings = ref<AppSettings>({ ...appDefaultSettings });

  return {
    settings,
  };
});
