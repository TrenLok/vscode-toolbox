import type { Project } from '~/types/project';

export function getProjectPath(project: Pick<Project, 'folder' | 'uri'>): string {
  return project.uri ?? project.folder;
}
