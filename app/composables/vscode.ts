import type { VSCodeVersion } from '~/types/vscode';

const VSCODE_COMMANDS = {
  windows: {
    openProject: ['vscode-cli-win-folder'],
    getVersion: ['vscode-cli-win-v'],
  },
  macos: {
    openProject: ['vscode-cli-mac-path-folder'],
    getVersion: ['vscode-cli-mac-path-v'],
  },
} as const;

type SupportedPlatform = keyof typeof VSCODE_COMMANDS;
type VSCodeCommandType = keyof typeof VSCODE_COMMANDS.windows;

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

  async function executeVSCodeCommand(commandType: VSCodeCommandType, args: string[]) {
    const platform = getSupportedPlatform();
    const commandNames = VSCODE_COMMANDS[platform][commandType];
    const errors: string[] = [];

    for (const commandName of commandNames) {
      try {
        return await useTauriShellCommand.create(commandName, args).execute();
      } catch (error_) {
        errors.push(`${commandName}: ${getErrorMessage(error_)}`);
      }
    }

    throw new Error(`Couldn't execute VS Code command "${commandType}" on ${platform}. Tried: ${errors.join(' | ')}`);
  }

  async function openProject(folder: string) {
    try {
      await executeVSCodeCommand('openProject', ['--', folder]);
    } catch (error_) {
      useTauriLogError(`Couldn't launch vscode: ${getErrorMessage(error_)}`);
    }
  }

  async function getVersion(): Promise<VSCodeVersion | undefined> {
    try {
      const out = await executeVSCodeCommand('getVersion', ['-v']);
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
