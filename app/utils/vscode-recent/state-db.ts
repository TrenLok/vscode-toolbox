import { invoke } from '@tauri-apps/api/core';
import { configDir, homeDir, join } from '@tauri-apps/api/path';
import type { OpenedPathsList, VSCodeRecentProject } from '~/types/vscode-recent';
import { parseOpenedPathsList } from '~/utils/vscode-recent/parser';

interface VSCodeProduct {
  configDirName: string;
  sharedDataDirName: string;
}

const VSCODE_PRODUCTS: VSCodeProduct[] = [
  { configDirName: 'Code', sharedDataDirName: '.vscode-shared' },
  { configDirName: 'Code - Insiders', sharedDataDirName: '.vscode-insiders-shared' },
  { configDirName: 'VSCodium', sharedDataDirName: '.vscodium-shared' },
];

export async function getRecentProjects(): Promise<VSCodeRecentProject[]> {
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

export async function getCandidateStateDbPaths(): Promise<string[]> {
  const configBase = await configDir();
  const homeBase = await homeDir();
  const candidates: string[] = [];

  for (const product of VSCODE_PRODUCTS) {
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

export function isVSCodeStateDbPath(path: string): boolean {
  return path.endsWith('state.vscdb')
    || path.endsWith('state.vscdb-wal')
    || path.endsWith('state.vscdb-shm')
    || path.endsWith('state.vscdb-journal');
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
