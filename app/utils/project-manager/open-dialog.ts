import { invoke } from '@tauri-apps/api/core';

export async function openProjectFolderDialog(): Promise<null | string> {
  const isMacos = useTauriOsPlatform() === 'macos';
  let autoHideSuspended = false;

  try {
    if (isMacos) {
      await invoke('suspend_window_auto_hide');
      autoHideSuspended = true;
    }

    const folder = await useTauriDialogOpen({
      multiple: false,
      directory: true,
    });

    return folder;
  } finally {
    if (autoHideSuspended) {
      try {
        await invoke('resume_window_auto_hide');
      } catch (error_) {
        useTauriLogError(`Couldn't restore window auto hide: ${error_}`);
      }
    }
  }
}
