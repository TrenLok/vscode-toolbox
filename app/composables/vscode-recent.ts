/* eslint-disable ts/no-explicit-any */
import { invoke } from '@tauri-apps/api/core';
import { configDir, join } from '@tauri-apps/api/path';
import type { OpenedPathsList, VSCodeRecentProject } from '~/types/vscode-recent';

let unwatchGlobal: null | (() => void) = null;

export function useVscodeRecent() {
  const PRODUCTS = ['Code', 'Code - Insiders', 'VSCodium'];

  function fileUriToFsPath(uri: string): string | null {
    try {
      if (!uri) return null;
      if (!/^file:\/\//i.test(uri)) {
        // ignoring remote vscode-remote schemas:// etc.
        return null;
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

    function addFolder(uri: string) {
      const path = fileUriToFsPath(uri);
      if (!path) return;

      const key = `folder:${path}`;
      if (seen.has(key)) return;

      seen.add(key);
      result.push({ type: 'folder', path: path });
    }

    const entries = paths?.entries;
    if (!Array.isArray(entries)) return result;

    for (const entry of entries) {
      if ('folderUri' in entry) {
        addFolder(entry.folderUri);
      }
    }

    return result;
  }

  async function findStateDbPath(): Promise<string | null> {
    const base = await configDir();
    for (const product of PRODUCTS) {
      const path = await join(base, product, 'User', 'globalStorage', 'state.vscdb');

      if (await useTauriFsExists(path)) return path;
    }
    return null;
  }

  async function getFolders() {
    const dbPath = await findStateDbPath();

    if (dbPath && await useTauriFsExists(dbPath)) {
      try {
        const jsonStr = await invoke<string>('get_vscode_recent_from_state', { dbPath });
        const paths = JSON.parse(jsonStr) as OpenedPathsList;
        const items = parseOpenedPathsList(paths);
        if (items.length) {
          return items.slice(0, 30).toReversed();
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

    const dbPath = await findStateDbPath();

    if (!dbPath) return null;

    const dir = await useTauriPathDirname(dbPath);
    const watchPaths: string[] = [dir];
    if (await useTauriFsExists(dbPath)) watchPaths.push(dbPath);

    if (watchPaths.length === 0) {
      throw new Error('No watchable paths found');
    }

    let timer: ReturnType<typeof setTimeout>;
    const trigger = () => {
      clearTimeout(timer);
      timer = setTimeout(onChange, debounceMs);
    };

    const unwatchFn = await useTauriFsWatch([dir, dbPath], (event: any) => {
      const events = Array.isArray(event) ? event : [event];

      for (const _event of events) {
        const paths = Array.isArray(_event.paths) ? _event.paths : [_event.path ?? _event.paths].filter(Boolean);
        if (!paths) continue;

        for (const path of paths) {
          if (typeof path === 'string' && path.endsWith('state.vscdb')) {
            trigger();
            return;
          }
        }
      }
    });

    unwatchGlobal = unwatchFn;

    function unwatch() {
      try {
        unwatchGlobal?.();
      } catch {
        //
      }
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

  function equalExact(
    a: VSCodeRecentProject[],
    b: VSCodeRecentProject[],
  ): boolean {
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) {
      if (a[i]?.type !== b[i]?.type || a[i]?.path !== b[i]?.path) return false;
    }
    return true;
  }

  return {
    getFolders, watchVSCodeState, equalExact,
  };
}
