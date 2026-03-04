import type { Project } from '~/types/project';

export function useProjectManager() {
  const vscode = useVscode();
  const projectStore = useProjectStore();
  const vscodeRecent = useVscodeRecent();
  const modals = useModals();

  const hiddenFolders = useHiddenFolders();

  const appStore = useAppStore();

  const projects = computed(() => projectStore.projects ?? []);
  const uniqueFolders = computed(() => projectStore.uniqueFolders);
  const badFolders = computed(() => projectStore.badFolders);
  const vscodeRecentItems = computed(() => projectStore.vscodeRecent ?? []);

  const favorites = computed<Project[]>(() => {
    return projects.value
      .filter((project) => project.is_favorite)
      .sort((a, b) => b.last_modified_timestamp - a.last_modified_timestamp);
  });
  const local = computed<Project[]>(() => {
    return projects.value
      .filter((project) => !project.is_favorite)
      .sort((a, b) => b.last_modified_timestamp - a.last_modified_timestamp);
  });

  const { save, load, clear } = useTauriStore<Project[]>({
    file: 'projects.json',
    key: 'projects',
    defaultValue: [],
    logPrefix: 'Project Manager',
    latestVersion: 1,
  });

  async function saveToDb() {
    await save(projects.value);
  }

  async function loadFromDb() {
    const items = await load();
    projectStore.projects = items;
  }

  async function clearDb() {
    projectStore.projects = [];
    await clear();
  }

  function getProjectByFolder(folder: string): Project | undefined {
    return projects.value.find((project) => project.folder === folder);
  }

  function checkProjectIsExists(folder: string): boolean {
    return projects.value.some((project) => project.folder === folder);
  }

  async function updateLastModifiedTimestamp(folder: string) {
    const project = getProjectByFolder(folder);

    if (!project) return;

    project.last_modified_timestamp = Date.now();

    await saveToDb();
  }

  async function changeFavorite(folder: string) {
    const project = getProjectByFolder(folder);
    if (!project) {
      useTauriLogInfo('The project could not be added to favorites because it was not found');
      useTauriNotificationSendNotification({
        title: 'Error',
        body: 'Couldn\'t add to favorites',
      });
      return;
    }

    project.is_favorite = !project.is_favorite;
    project.last_modified_timestamp = Date.now();

    await saveToDb();
  }

  async function addNewProject(folder: string) {
    const name = await useTauriPathBasename(folder);
    const project: Project = {
      name: name ?? 'Unknown name',
      folder,
      is_favorite: false,
      last_modified_timestamp: Date.now(),
    };

    projects.value.unshift(project);

    saveToDb();
  }

  async function openNewProject() {
    try {
      await vscode.getVersion();
    } catch {
      await modals.vsCodeNotInstalled();
      return;
    }

    const folder = await useTauriDialogOpen({
      multiple: false,
      directory: true,
    });

    if (!folder) return;

    const normalizedFolder = await getNormalizedAndResolvedFolderPath(folder);

    // Removes a directory from the list of hidden directories and bad folders if it exists when opened
    hiddenFolders.deleteFolder(normalizedFolder);
    badFolders.value.delete(normalizedFolder);

    await openProjectFolder(normalizedFolder);
  }

  async function openProjectFolder(folder: string) {
    try {
      await vscode.getVersion();
    } catch {
      await modals.vsCodeNotInstalled();
      return;
    }

    const folderIsExist = await useTauriFsExists(folder);
    if (!folderIsExist) {
      badFolders.value.add(folder);
      await modals.notFound(folder);
      useTauriLogInfo('Error opening folder: the specified folder does not exist');
      return;
    }

    // Removes a directory from the list of hidden directories if it exists when opened
    hiddenFolders.deleteFolder(folder);

    const projectExist = checkProjectIsExists(folder);
    if (!projectExist) {
      addNewProject(folder);
    }
    vscode.openProject(folder);
    updateLastModifiedTimestamp(folder);
    appStore.scrollToTop();
  }

  async function checkEqualsAndSyncVSCodeRecent() {
    const prev = vscodeRecentItems.value;
    const next = await vscodeRecent.getFolders();
    const changed = !vscodeRecent.equalExact(prev, next);

    if (!changed) return;

    await syncVSCodeRecent();
  }

  async function syncVSCodeRecent() {
    const recentVSCodeProjects = await vscodeRecent.getFolders();
    projectStore.vscodeRecent = recentVSCodeProjects;
    for (const project of recentVSCodeProjects) {
      const projectPath = await getNormalizedAndResolvedFolderPath(project.path);

      // Checks whether the directory is in the list of hidden directories
      // If the directory is in the list and exists, removes it from the list of hidden directories
      const hiddenFolder = hiddenFolders.getFolderByPath(projectPath);
      if (hiddenFolder?.isDeleted) {
        const folderIsExist = await useTauriFsExists(projectPath);
        if (folderIsExist) hiddenFolders.deleteFolder(projectPath);
      }

      if (!uniqueFolders.value.has(projectPath)) {
        addNewProject(projectPath);
      }
    }

    const firstRecent = recentVSCodeProjects.at(-1);

    if (firstRecent) {
      const projectPath = await getNormalizedAndResolvedFolderPath(firstRecent.path);
      if (uniqueFolders.value.has(projectPath)) updateLastModifiedTimestamp(projectPath);
    }

    await saveToDb();
  }

  async function checkBadFolders() {
    for (const folder of uniqueFolders.value) {
      const folderIsExist = await useTauriFsExists(folder);
      if (folderIsExist) {
        projectStore.badFolders.delete(folder);
      } else {
        projectStore.badFolders.add(folder);
      }
    }
  }

  return {
    projects: ref({
      favorites,
      local,
    }),
    uniqueFolders,
    badFolders,
    vscodeRecentItems,

    openNewProject,
    openProjectFolder,
    saveToDb,
    loadFromDb,
    clearDb,
    changeFavorite,
    addNewProject,
    checkEqualsAndSyncVSCodeRecent,
    syncVSCodeRecent,
    checkBadFolders,
  };
}
