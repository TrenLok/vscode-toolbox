import type { Project, ProjectType } from '~/types/project';

export function getProjectPath(project: Pick<Project, 'folder' | 'uri'>): string {
  return project.uri ?? project.folder;
}

export function isProjectWorkspacePath(path: string): boolean {
  return path.split(/[?#]/, 1)[0]?.toLowerCase().endsWith('.code-workspace') ?? false;
}

export function getProjectTypeFromPath(path: string): ProjectType {
  return isProjectWorkspacePath(path) ? 'workspace' : 'folder';
}

export function shouldStoreProjectUri(openPath: string, type?: ProjectType): boolean {
  return isVSCodeRemoteUri(openPath) || type === 'workspace';
}
