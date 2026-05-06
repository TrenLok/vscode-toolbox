/* eslint-disable ts/no-explicit-any */
import { invoke } from '@tauri-apps/api/core';
import { configDir, homeDir, join } from '@tauri-apps/api/path';
import type { OpenedPathsList, VSCodeRecentEntryFolder, VSCodeRecentProject } from '~/types/vscode-recent';

let unwatchGlobal: null | (() => void) = null;

interface VSCodeProduct {
  configDirName: string;
  sharedDataDirName: string;
}

interface VSCodeRecentDisplay {
  folder: string;
  name: string;
}

export function useVscodeRecent() {
  const PRODUCTS: VSCodeProduct[] = [
    { configDirName: 'Code', sharedDataDirName: '.vscode-shared' },
    { configDirName: 'Code - Insiders', sharedDataDirName: '.vscode-insiders-shared' },
    { configDirName: 'VSCodium', sharedDataDirName: '.vscodium-shared' },
  ];

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

  function parseOpenedPathsList(paths: OpenedPathsList): VSCodeRecentProject[] {
    const result: VSCodeRecentProject[] = [];
    const seen = new Set<string>();

    function addFolder(entry: VSCodeRecentEntryFolder) {
      const uri = entry.folderUri;
      const path = uriToProjectPath(uri);
      if (!path) return;

      const key = `folder:${path}`;
      if (seen.has(key)) return;

      let display: null | VSCodeRecentDisplay = null;
      if (isVSCodeRemoteUri(path)) {
        display = entry.label ? getVSCodeRemoteDisplay(entry.label) : null;
        display ??= getVSCodeRemoteDisplayFromUri(path);
      }
      const project: VSCodeRecentProject = {
        type: 'folder',
        path,
      };

      seen.add(key);
      if (display) {
        project.name = display.name;
        project.folder = display.folder;
      }
      result.push(project);
    }

    const entries = paths?.entries;
    if (!Array.isArray(entries)) return result;

    for (const entry of entries) {
      if ('folderUri' in entry) {
        addFolder(entry);
      }
    }

    return result;
  }

  async function getCandidateStateDbPaths(): Promise<string[]> {
    const configBase = await configDir();
    const homeBase = await homeDir();
    const candidates: string[] = [];

    for (const product of PRODUCTS) {
      candidates.push(
        await join(homeBase, product.sharedDataDirName, 'sharedStorage', 'state.vscdb'),
        await join(configBase, product.configDirName, 'User', 'globalStorage', 'state.vscdb'),
      );
    }

    const uniqueCandidates: string[] = [];
    const seen = new Set<string>();

    for (const path of candidates) {
      if (seen.has(path)) continue;
      seen.add(path);
      uniqueCandidates.push(path);
    }

    return uniqueCandidates;
  }

  async function getStateDbPaths(): Promise<string[]> {
    const dbPaths: string[] = [];

    for (const path of await getCandidateStateDbPaths()) {
      if (
        await useTauriFsExists(path)
        && await invoke<boolean>('has_vscode_recent_state_key', { dbPath: path })
      ) {
        dbPaths.push(path);
      }
    }

    return dbPaths;
  }

  async function getFolders() {
    const dbPaths = await getStateDbPaths();
    for (const dbPath of dbPaths) {
      try {
        const jsonStr = await invoke<string>('get_vscode_recent_from_state', { dbPath });
        const paths = JSON.parse(jsonStr) as OpenedPathsList;
        const items = parseOpenedPathsList(paths);
        if (items.length) {
          // eslint-disable-next-line unicorn/no-array-reverse
          return items.reverse();
        }
      } catch (error) {
        useTauriLogError(`Couldn't get the last open folders: ${error}`);
      }
    }

    return [];
  }

  async function watchVSCodeState(
    onChange: () => void,
    debounceMs = 150,
  ): Promise<null | { unwatch: () => void }> {
    if (unwatchGlobal) {
      try {
        unwatchGlobal();
      } catch {
        //
      }
      unwatchGlobal = null;
    }

    const watchPaths: string[] = [];
    const seenWatchPaths = new Set<string>();
    for (const dbPath of await getCandidateStateDbPaths()) {
      const dir = await useTauriPathDirname(dbPath);
      const paths = [
        await useTauriFsExists(dir) ? dir : null,
        await useTauriFsExists(dbPath) ? dbPath : null,
      ].filter(Boolean) as string[];

      for (const path of paths) {
        if (seenWatchPaths.has(path)) continue;
        seenWatchPaths.add(path);
        watchPaths.push(path);
      }
    }

    if (watchPaths.length === 0) {
      throw new Error('No watchable paths found');
    }

    let timer: ReturnType<typeof setTimeout>;
    const trigger = () => {
      clearTimeout(timer);
      timer = setTimeout(onChange, debounceMs);
    };

    const unwatchFn = await useTauriFsWatch(watchPaths, (event: any) => {
      const events = Array.isArray(event) ? event : [event];

      for (const _event of events) {
        const paths = Array.isArray(_event.paths) ? _event.paths : [_event.path ?? _event.paths].filter(Boolean);
        if (!paths) continue;

        for (const path of paths) {
          if (typeof path === 'string' && isVSCodeStateDbPath(path)) {
            trigger();
            return;
          }
        }
      }
    });

    unwatchGlobal = unwatchFn;

    function unwatch() {
      try {
        unwatchFn();
      } catch {
        //
      }
      clearTimeout(timer);
      unwatchGlobal = null;
      window.removeEventListener('beforeunload', unwatch);
    }
    window.addEventListener('beforeunload', unwatch);

    if (import.meta.hot) {
      import.meta.hot.dispose(() => {
        try {
          unwatchGlobal?.();
        } catch {
          //
        }
        unwatchGlobal = null;
      });
    }

    return { unwatch };
  }

  function isVSCodeStateDbPath(path: string): boolean {
    return path.endsWith('state.vscdb')
      || path.endsWith('state.vscdb-wal')
      || path.endsWith('state.vscdb-shm')
      || path.endsWith('state.vscdb-journal');
  }

  function equalExact(
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

  return {
    getFolders, watchVSCodeState, equalExact,
  };
}
