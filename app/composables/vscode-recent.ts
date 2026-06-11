import { equalExact } from '~/utils/vscode-recent/parser';
import { getRecentProjects } from '~/utils/vscode-recent/state-db';
import { watchVSCodeState } from '~/utils/vscode-recent/watcher';

export function useVscodeRecent() {
  return {
    getRecentProjects,
    watchVSCodeState,
    equalExact,
  };
}
