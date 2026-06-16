import type {
  OpenedPathsList,
  VSCodeRecentEntryFolder,
  VSCodeRecentEntryWorkspace,
  VSCodeRecentProject,
} from '~/types/vscode-recent';
import {
  getVSCodeRemoteDisplay,
  getVSCodeRemoteDisplayFromUri,
  isVSCodeRemoteUri,
} from '~/utils/fs';

interface VSCodeRecentDisplay {
  folder: string;
  name: string;
}

export function parseOpenedPathsList(paths: OpenedPathsList): VSCodeRecentProject[] {
  const result: VSCodeRecentProject[] = [];
  const seen = new Set<string>();

  function addProject(project: VSCodeRecentProject) {
    const key = `${project.type}:${project.path}`;
    if (seen.has(key)) return;

    seen.add(key);
    result.push(project);
  }

  const entries = paths?.entries;
  if (!Array.isArray(entries)) return result;

  for (const entry of entries) {
    if ('folderUri' in entry) {
      const project = createFolderProject(entry);
      if (project) addProject(project);
    } else if ('workspace' in entry) {
      const project = createWorkspaceProject(entry);
      if (project) addProject(project);
    }
  }

  return result;
}

export function equalExact(
  a: VSCodeRecentProject[],
  b: VSCodeRecentProject[],
): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (
      a[i]?.type !== b[i]?.type
      || a[i]?.path !== b[i]?.path
      || a[i]?.name !== b[i]?.name
      || a[i]?.folder !== b[i]?.folder
    ) return false;
  }
  return true;
}

function createFolderProject(entry: VSCodeRecentEntryFolder): null | VSCodeRecentProject {
  const uri = entry.folderUri;
  const path = uriToProjectPath(uri);
  if (!path) return null;

  let display: null | VSCodeRecentDisplay = null;
  if (isVSCodeRemoteUri(path)) {
    display = entry.label ? getVSCodeRemoteDisplay(entry.label) : null;
    display ??= getVSCodeRemoteDisplayFromUri(path);
  }

  const project: VSCodeRecentProject = {
    type: 'folder',
    path,
  };

  if (display) {
    project.name = display.name;
    project.folder = display.folder;
  }

  return project;
}

function createWorkspaceProject(entry: VSCodeRecentEntryWorkspace): null | VSCodeRecentProject {
  const uri = entry.workspace.configPath ?? entry.workspace.configURIPath;
  if (!uri) return null;

  const path = uriToProjectPath(uri) ?? uri;
  if (!path) return null;

  const project: VSCodeRecentProject = {
    type: 'workspace',
    path,
    folder: getPathDirname(path),
    name: getWorkspaceName(path),
  };

  if (isVSCodeRemoteUri(path)) {
    const display = getVSCodeRemoteWorkspaceDisplay(path, entry.label);

    if (display) {
      project.folder = display.folder;
      project.name = display.name;
    }
  }

  return project;
}

function uriToProjectPath(uri: string): string | null {
  try {
    if (!uri) return null;
    if (!/^file:\/\//i.test(uri)) {
      return isVSCodeRemoteUri(uri) ? uri : null;
    }
    const url = new URL(uri);
    let path = decodeURIComponent(url.pathname);

    if (isLikelyWindowsPath(path)) {
      if (path.startsWith('/')) path = path.slice(1);
      path = path.replaceAll('/', '\\');
    }
    return path;
  } catch {
    return null;
  }
}

function isLikelyWindowsPath(pathname: string): boolean {
  // /C:/... or /c:/...
  return /^\/[a-z]:\//i.test(pathname);
}

// Extracts the parent directory from Windows and POSIX-style local paths.
function getPathDirname(path: string): string {
  const separator = path.includes('\\') || /^[a-z]:/i.test(path) ? '\\' : '/';
  const normalizedPath = separator === '\\' ? path.replaceAll('/', '\\') : path;
  const index = normalizedPath.lastIndexOf(separator);

  return index === -1 ? path : normalizedPath.slice(0, index);
}

// Uses the .code-workspace filename as the display name.
function getWorkspaceName(path: string): string {
  const cleanPath = path.split(/[?#]/, 1)[0] ?? path;
  const filename = cleanPath.split(/[\\/]/).at(-1) ?? cleanPath;

  return safeDecodeURIComponent(filename).replace(/\.code-workspace$/i, '');
}

function getVSCodeRemoteWorkspaceDisplay(uri: string, label?: string): null | VSCodeRecentDisplay {
  const labelDisplay = label ? getVSCodeRemoteWorkspaceDisplayFromLabel(label) : null;
  const folderPath = getPathDirname(uri);
  const folderDisplay = getVSCodeRemoteDisplayFromUri(folderPath);
  const workspaceDisplay = getVSCodeRemoteDisplayFromUri(uri);
  const coder = labelDisplay?.name.match(/\s*(\[Coder:[^\]]+\])/)?.at(1)
    ?? workspaceDisplay?.name.match(/\s*(\[Coder:[^\]]+\])/)?.at(1)
    ?? '';
  const name = [getWorkspaceName(uri), coder].filter(Boolean).join(' ');

  return name
    ? { name, folder: labelDisplay?.folder ?? folderDisplay?.folder ?? folderPath }
    : null;
}

function getVSCodeRemoteWorkspaceDisplayFromLabel(label: string): null | VSCodeRecentDisplay {
  const display = getVSCodeRemoteDisplay(label);
  if (!display) return null;

  return {
    folder: stripWorkspaceLabel(display.folder),
    name: stripWorkspaceLabel(display.name),
  };
}

function stripWorkspaceLabel(value: string): string {
  return value.replace(/\s+\(Workspace\)(?=\s*(?:\[[^\]]+\])?$)/i, '').trim();
}

function safeDecodeURIComponent(value: string): string {
  try {
    return decodeURIComponent(value);
  } catch {
    return value;
  }
}
