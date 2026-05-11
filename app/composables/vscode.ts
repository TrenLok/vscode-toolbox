import type { VSCodeVersion } from '~/types/vscode';
import { invoke } from '@tauri-apps/api/core';

type SupportedPlatform = 'windows' | 'macos';

export function useVscode() {
  function getErrorMessage(error_: unknown): string {
    if (error_ instanceof Error) return `${error_.name}: ${error_.message}`;
    if (typeof error_ === 'string') return error_;

    try {
      return JSON.stringify(error_, null, 2);
    } catch {
      return String(error_);
    }
  }

  function getSupportedPlatform(): SupportedPlatform {
    const platform = useTauriOsPlatform();

    if (platform === 'windows' || platform === 'macos') return platform;

    throw new Error(`Unsupported platform for VS Code integration: ${platform}`);
  }

  async function openProject(folder: string) {
    try {
      const platform = getSupportedPlatform();

      if (platform === 'macos') {
        if (isVSCodeRemoteUri(folder)) {
          await invoke('open_vscode_project_uri_macos', { uri: folder });
        } else {
          await invoke('open_vscode_project_macos', { folder });
        }
      } else if (isVSCodeRemoteUri(folder)) {
        await invoke('open_vscode_project_uri_windows', { uri: folder });
      } else {
        await invoke('open_vscode_project_windows', { folder });
      }
    } catch (error_) {
      useTauriLogError(`Couldn't launch vscode: ${getErrorMessage(error_)}`);
    }
  }

  async function getVersion(): Promise<VSCodeVersion | undefined> {
    try {
      const platform = getSupportedPlatform();

      if (platform === 'macos') {
        return await invoke<VSCodeVersion>('get_vscode_version_macos');
      }

      return await invoke<VSCodeVersion>('get_vscode_version_windows');
    } catch (error_) {
      useTauriLogError(`Couldn't determine the vscode version: ${getErrorMessage(error_)}`);

      throw error_;
    }
  }

  return {
    openProject, getVersion,
  };
}
