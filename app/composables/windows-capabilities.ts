import { invoke } from '@tauri-apps/api/core';

export interface WindowsCapabilities {
  build: number | null;
  isUndocumentedMicaSupported: boolean;
  isBackdroptypeSupported: boolean;
  isMicaSupported: boolean;
}

export function useWindowsCapabilities(): Promise<WindowsCapabilities> {
  return invoke<WindowsCapabilities>('windows_capabilities');
}
