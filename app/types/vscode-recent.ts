import type { ProjectType } from '~/types/project';

export interface VSCodeRecentEntryFolder {
  folderUri: string;
  label?: string;
  remoteAuthority?: string;
}

interface EntryFile {
  fileUri: string;
}

export interface VSCodeRecentEntryWorkspace {
  workspace: {
    configPath?: string;
    configURIPath?: string;
  };
  label?: string;
  remoteAuthority?: string;
}

export interface OpenedPathsList {
  entries?: (VSCodeRecentEntryFolder | EntryFile | VSCodeRecentEntryWorkspace)[];
}

export interface VSCodeRecentProject {
  type: ProjectType;
  path: string;
  name?: string;
  folder?: string;
}
