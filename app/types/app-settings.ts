export type AppTheme = 'default' | 'mica' | 'liquid_glass';

export interface AppSettings {
  vsCodeSync: boolean;
  autoCheckUpdates: boolean;
  theme: AppTheme;
}
