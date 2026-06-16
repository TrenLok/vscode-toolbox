import {
  getCandidateStateDbPaths,
  isVSCodeStateDbPath,
} from '~/utils/vscode-recent/state-db';

type Unwatch = () => void;

let unwatchGlobal: null | Unwatch = null;

export async function watchVSCodeState(
  onChange: () => void,
  debounceMs = 150,
): Promise<null | { unwatch: Unwatch }> {
  disposeGlobalWatcher();

  const watchPaths = await getWatchPaths();
  if (watchPaths.length === 0) {
    throw new Error('No watchable paths found');
  }

  const debounce = createDebouncedTrigger(onChange, debounceMs);

  const unwatchFn = await useTauriFsWatch(watchPaths, (event: unknown) => {
    for (const path of getWatchEventPaths(event)) {
      if (isVSCodeStateDbPath(path)) {
        debounce.trigger();
        return;
      }
    }
  });

  unwatchGlobal = unwatchFn;

  function unwatch() {
    safelyUnwatch(unwatchFn);
    debounce.clear();
    unwatchGlobal = null;
    window.removeEventListener('beforeunload', unwatch);
  }
  window.addEventListener('beforeunload', unwatch);

  if (import.meta.hot) {
    import.meta.hot.dispose(() => {
      disposeGlobalWatcher();
    });
  }

  return { unwatch };
}

function disposeGlobalWatcher() {
  if (!unwatchGlobal) return;

  safelyUnwatch(unwatchGlobal);
  unwatchGlobal = null;
}

function safelyUnwatch(unwatch: Unwatch) {
  try {
    unwatch();
  } catch {
    //
  }
}

async function getWatchPaths(): Promise<string[]> {
  const watchPaths: string[] = [];
  const seenWatchPaths = new Set<string>();

  for (const dbPath of await getCandidateStateDbPaths()) {
    const dir = await useTauriPathDirname(dbPath);
    const paths = [
      await useTauriFsExists(dir) ? dir : null,
      await useTauriFsExists(dbPath) ? dbPath : null,
    ].filter((path): path is string => typeof path === 'string');

    for (const path of paths) {
      if (seenWatchPaths.has(path)) continue;
      seenWatchPaths.add(path);
      watchPaths.push(path);
    }
  }

  return watchPaths;
}

function createDebouncedTrigger(onChange: () => void, debounceMs: number) {
  let timer: ReturnType<typeof setTimeout> | undefined;

  return {
    trigger() {
      clearTimeout(timer);
      timer = setTimeout(onChange, debounceMs);
    },
    clear() {
      clearTimeout(timer);
      timer = undefined;
    },
  };
}

function getWatchEventPaths(event: unknown): string[] {
  const events = Array.isArray(event) ? event : [event];

  return events.flatMap((singleEvent) => getSingleWatchEventPaths(singleEvent));
}

function getSingleWatchEventPaths(event: unknown): string[] {
  if (!event || typeof event !== 'object') return [];
  const { path, paths } = event as { path?: unknown; paths?: unknown };
  const values = Array.isArray(paths) ? paths : [path ?? paths];

  return values.filter((value): value is string => typeof value === 'string');
}
