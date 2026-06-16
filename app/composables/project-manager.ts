import type { Project } from '~/types/project';
import { invoke } from '@tauri-apps/api/core';

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

  function getProjectByOpenPath(openPath: string): Project | undefined {
    return projects.value.find((project) => (project.uri ?? project.folder) === openPath);
  }

  function checkProjectIsExists(openPath: string): boolean {
    return Boolean(getProjectByOpenPath(openPath));
  }

  function resolveProjectOpenPath(folder: string, uri = folder): string {
    if (isVSCodeRemoteUri(uri)) return uri;

    const storedUri = getProjectByFolder(folder)?.uri;
    if (storedUri) return storedUri;

    // Migration fallback for projects saved before remote URI was stored separately.
    const recentProject = vscodeRecentItems.value.find((project) => {
      return project.folder === folder && isVSCodeRemoteUri(project.path);
    });

    return recentProject?.path ?? uri;
  }

  async function updateLastModifiedTimestamp(openPath: string) {
    const project = getProjectByOpenPath(openPath);

    if (!project) return;

    project.last_modified_timestamp = Date.now();

    await saveToDb();
  }

  async function changeFavorite(openPath: string) {
    const project = getProjectByOpenPath(openPath);
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

  async function addNewProject(
    openPath: string,
    display: Partial<Pick<Project, 'folder' | 'name'>> = {},
  ) {
    const folder = display.folder ?? openPath;
    const name = display.name
      ?? (isVSCodeRemoteUri(openPath)
        ? getProjectFolderName(openPath)
        : await useTauriPathBasename(folder));
    const project: Project = {
      name: name ?? 'Unknown name',
      folder,
      ...(isVSCodeRemoteUri(openPath) ? { uri: openPath } : {}),
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

    const isMacos = useTauriOsPlatform() === 'macos';
    let autoHideSuspended = false;

    try {
      if (isMacos) {
        await invoke('suspend_window_auto_hide');
        autoHideSuspended = true;
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
    } finally {
      if (autoHideSuspended) {
        try {
          await invoke('resume_window_auto_hide');
        } catch (error_) {
          useTauriLogError(`Couldn't restore window auto hide: ${error_}`);
        }
      }
    }
  }

  async function openProjectFolder(folder: string, uri = folder) {
    const openPath = resolveProjectOpenPath(folder, uri);

    try {
      await vscode.getVersion();
    } catch {
      await modals.vsCodeNotInstalled();
      return;
    }

    if (!isVSCodeRemoteUri(openPath) && !await useTauriFsExists(folder)) {
      badFolders.value.add(folder);
      await modals.notFound(folder);
      useTauriLogInfo('Error opening folder: the specified folder does not exist');
      return;
    }

    // Removes a project from the list of hidden projects if it exists when opened
    hiddenFolders.deleteFolder(openPath);
    hiddenFolders.deleteFolder(folder);

    const projectExist = checkProjectIsExists(openPath);
    if (!projectExist) {
      addNewProject(openPath, { folder });
    }

    try {
      await invoke('hide_current_window');
    } catch (error_) {
      useTauriLogError(`Couldn't hide toolbox before opening project: ${error_}`);
    }

    const didOpen = await vscode.openProject(openPath);
    if (!didOpen) return;

    updateLastModifiedTimestamp(openPath);
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
      const projectFolder = project.folder ?? projectPath;

      // Checks whether the directory is in the list of hidden directories
      // If the directory is in the list and exists, removes it from the list of hidden directories
      const hiddenFolder = hiddenFolders.getFolderByPath(projectPath)
        ?? hiddenFolders.getFolderByPath(projectFolder);
      const canRestoreHiddenFolder = isVSCodeRemoteUri(projectPath) || await useTauriFsExists(projectPath);
      if (hiddenFolder?.isDeleted && canRestoreHiddenFolder) {
        hiddenFolders.deleteFolder(projectPath);
        hiddenFolders.deleteFolder(projectFolder);
      }

      const display = { name: project.name, folder: projectFolder };
      const existingProject = getProjectByOpenPath(projectPath);

      if (existingProject) {
        existingProject.name = display.name ?? existingProject.name;
        existingProject.folder = display.folder;
        existingProject.uri = isVSCodeRemoteUri(projectPath) ? projectPath : undefined;
      } else if (!uniqueFolders.value.has(projectPath)) {
        addNewProject(projectPath, display);
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
      if (isVSCodeRemoteUri(folder) || await useTauriFsExists(folder)) {
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
