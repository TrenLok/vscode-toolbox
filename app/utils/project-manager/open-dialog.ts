import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

export async function openProjectFolderDialog(): Promise<null | string> {
  const isMacos = useTauriOsPlatform() === 'macos';
  let autoHideSuspended = false;
  let restoreAlwaysOnTop = false;
  const appWindow = isMacos ? getCurrentWebviewWindow() : null;

  try {
    if (isMacos) {
      await invoke('suspend_window_auto_hide');
      autoHideSuspended = true;

      try {
        const isAlwaysOnTop = await appWindow?.isAlwaysOnTop();

        if (!isAlwaysOnTop) {
          await appWindow?.setAlwaysOnTop(true);
          restoreAlwaysOnTop = true;
        }
      } catch (error_) {
        useTauriLogError(`Couldn't enable always on top for folder dialog: ${error_}`);
      }
    }

    const folder = await useTauriDialogOpen({
      multiple: false,
      directory: true,
    });

    if (restoreAlwaysOnTop) {
      try {
        await appWindow?.setAlwaysOnTop(false);
        restoreAlwaysOnTop = false;
      } catch (error_) {
        useTauriLogError(`Couldn't restore always on top after folder dialog: ${error_}`);
      }
    }

    return folder;
  } finally {
    if (restoreAlwaysOnTop) {
      try {
        await appWindow?.setAlwaysOnTop(false);
      } catch (error_) {
        useTauriLogError(`Couldn't restore always on top after folder dialog: ${error_}`);
      }
    }

    if (autoHideSuspended) {
      try {
        await invoke('resume_window_auto_hide');
      } catch (error_) {
        useTauriLogError(`Couldn't restore window auto hide: ${error_}`);
      }
    }
  }
}
