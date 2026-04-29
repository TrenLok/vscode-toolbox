import type { AppTheme } from '~/types/app-settings';
import { invoke } from '@tauri-apps/api/core';

export function useAvailableThemes(): Promise<AppTheme[]> {
  return invoke<AppTheme[]>('available_themes');
}
