import type { Update } from '@tauri-apps/plugin-updater';

export const useAppUpdateStore = defineStore('AppUpdate', () => {
  const latestUpdate = ref<Update | null>(null);
  const updateIsChecked = ref(false);

  return {
    latestUpdate,
    updateIsChecked,
  };
});
