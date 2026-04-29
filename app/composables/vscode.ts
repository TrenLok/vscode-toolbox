import type { VSCodeVersion } from '~/types/vscode';
import { invoke } from '@tauri-apps/api/core';

const WINDOWS_VSCODE_COMMANDS = {
  openProject: ['vscode-cli-win-folder'],
  getVersion: ['vscode-cli-win-v'],
} as const;

type SupportedPlatform = 'windows' | 'macos';
type VSCodeCommandType = keyof typeof WINDOWS_VSCODE_COMMANDS;

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

  async function executeWindowsVSCodeCommand(commandType: VSCodeCommandType, args: string[]) {
    const commandNames = WINDOWS_VSCODE_COMMANDS[commandType];
    const errors: string[] = [];

    for (const commandName of commandNames) {
      try {
        return await useTauriShellCommand.create(commandName, args).execute();
      } catch (error_) {
        errors.push(`${commandName}: ${getErrorMessage(error_)}`);
      }
    }

    throw new Error(`Couldn't execute VS Code command "${commandType}" on windows. Tried: ${errors.join(' | ')}`);
  }

  async function openProject(folder: string) {
    try {
      const platform = getSupportedPlatform();

      if (platform === 'macos') {
        await invoke('open_vscode_project_macos', { folder });
      } else {
        await executeWindowsVSCodeCommand('openProject', ['--', folder]);
      }
    } catch (error_) {
      useTauriLogError(`Couldn't launch vscode: ${getErrorMessage(error_)}`);
    }
  }

  async function getVersion(): Promise<VSCodeVersion | undefined> {
    try {
      const platform = getSupportedPlatform();

      if (platform === 'macos') {
        const version = await invoke<string>('get_vscode_version_macos');

        return { version };
      }

      const out = await executeWindowsVSCodeCommand('getVersion', ['-v']);
      const versionInfo = out.stdout.trim().split(/\r?\n/);

      return {
        version: versionInfo[0],
        commit: versionInfo[1],
        architecture: versionInfo[2],
      };
    } catch (error_) {
      useTauriLogError(`Couldn't determine the vscode version: ${getErrorMessage(error_)}`);

      throw error_;
    }
  }

  return {
    openProject, getVersion,
  };
}
