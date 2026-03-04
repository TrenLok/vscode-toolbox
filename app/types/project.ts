export interface Project {
  name: string;
  folder: string;
  is_favorite: boolean;
  last_modified_timestamp: number;
}

export interface HiddenFolder {
  path: string;
  isDeleted: boolean;
}
