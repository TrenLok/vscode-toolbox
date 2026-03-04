interface EntryFolder {
  folderUri: string;
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
  entries?: (EntryFolder | EntryFile | EntryWorkspace)[];
}

export interface VSCodeRecentProject {
  type: 'folder' | 'workspace';
  path: string;
}
