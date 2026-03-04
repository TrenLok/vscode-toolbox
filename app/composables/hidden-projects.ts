import type { HiddenFolder } from '~/types/project';

export function useHiddenFolders() {
  const hiddenFoldersStore = useHiddenFoldersStore();

  const folders = computed(() => hiddenFoldersStore.folders);

  const { save, load, clear } = useTauriStore<HiddenFolder[]>({
    file: 'hidden_folders.json',
    key: 'folders',
    defaultValue: [],
    logPrefix: 'Hidden folders',
    latestVersion: 1,
  });

  async function saveToDb() {
    await save(folders.value);
  }

  async function loadFromDb() {
    const items = await load();
    hiddenFoldersStore.folders = items;
  }

  async function clearDb() {
    hiddenFoldersStore.folders = [];
    await clear();
  }

  async function addFolder(folder: HiddenFolder) {
    hiddenFoldersStore.folders.push(folder);

    await saveToDb();
  }

  async function deleteFolder(path: string) {
    const items = hiddenFoldersStore.folders.filter((item) => item.path !== path);
    hiddenFoldersStore.folders = items;

    await saveToDb();
  }

  function hasFolder(path: string): boolean {
    return folders.value.some((item) => item.path === path);
  }

  function getFolderByPath(path: string): HiddenFolder | undefined {
    return folders.value.findLast((item) => item.path === path);
  }

  return {
    folders,
    clearDb,
    loadFromDb,
    addFolder,
    deleteFolder,
    hasFolder,
    getFolderByPath,
  };
}
