import type { HiddenFolder } from '~/types/project';

export const useHiddenFoldersStore = defineStore('hidden-folders', () => {
  const folders = ref<HiddenFolder[]>([]);

  return {
    folders,
  };
});
