export type ProjectType = 'folder' | 'workspace';

export interface Project {
  type?: ProjectType;
  name: string;
  folder: string;
  uri?: string;
  is_favorite: boolean;
  last_modified_timestamp: number;
}

export type ProjectDisplay = Partial<Pick<Project, 'folder' | 'name' | 'type'>>;

export interface HiddenFolder {
  path: string;
  isDeleted: boolean;
}
