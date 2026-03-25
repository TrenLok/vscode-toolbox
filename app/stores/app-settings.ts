import type { AppSettings } from '~/types/app-settings';

export const appDefaultSettings: AppSettings = {
  vsCodeSync: false,
  autoCheckUpdates: false,
  theme: 'default',
};

export const useAppSettingsStore = defineStore('AppSettings', () => {
  const settings = ref<AppSettings>({ ...appDefaultSettings });

  return {
    settings,
  };
});
