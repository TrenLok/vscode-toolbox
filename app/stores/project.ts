import type { Project } from '~/types/project';
import type { VSCodeRecentProject } from '~/types/vscode-recent';

export const useProjectStore = defineStore('project', () => {
  const projects = ref<Project[]>([]);
  const vscodeRecent = ref<VSCodeRecentProject[]>([]);

  const uniqueFolders = computed<Set<string>>(() => {
    const folders = new Set<string>();
    projects.value.forEach((project) => {
      folders.add(normalizeWindowsPathDrive(project.folder));
    });
    return folders;
  });

  const badFolders = ref<Set<string>>(new Set<string>());

  return {
    projects,
    uniqueFolders,
    badFolders,
    vscodeRecent,
  };
});
