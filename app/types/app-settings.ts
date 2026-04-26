export type AppTheme = 'default' | 'mica' | 'liquid_glass' | 'vibrancy';

export interface AppSettings {
  vsCodeSync: boolean;
  autoCheckUpdates: boolean;
  theme: AppTheme;
}
