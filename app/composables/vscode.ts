import type { VSCodeVersion } from '~/types/vscode';

export function useVscode() {
  async function openProject(folder: string) {
    try {
      await useTauriShellCommand.create('vscode-cli-win-folder', ['--', folder]).execute();
    } catch (error_) {
      useTauriLogError(`Couldn't launch vscode: ${JSON.stringify(error_, null, 2)}`);
    }
  }

  async function getVersion(): Promise<VSCodeVersion | undefined> {
    try {
      const out = await useTauriShellCommand.create('vscode-cli-win-v', ['-v']).execute();
      const versionInfo = out.stdout.trim().split('\n');

      return {
        version: versionInfo[0],
        commit: versionInfo[1],
        architecture: versionInfo[2],
      };
    } catch (error_) {
      useTauriLogError(`Couldn't determine the vscode version: ${JSON.stringify(error_, null, 2)}`);

      throw error_;
    }
  }

  return {
    openProject, getVersion,
  };
}
