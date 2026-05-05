export interface VSCodeRecentEntryFolder {
  folderUri: string;
  label?: string;
  remoteAuthority?: string;
}

interface EntryFile {
  fileUri: string;
}

interface EntryWorkspace {
  workspace: {
    configURIPath: string;
  };
}

export interface OpenedPathsList {
  entries?: (VSCodeRecentEntryFolder | EntryFile | EntryWorkspace)[];
}

export interface VSCodeRecentProject {
  type: 'folder' | 'workspace';
  path: string;
  name?: string;
  folder?: string;
}
