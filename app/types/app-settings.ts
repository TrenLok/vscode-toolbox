export type AppTheme = 'default' | 'mica' | 'liquid_glass' | 'vibrancy';
export type ProjectIconStyle = 'default' | 'gradient';

export interface AppSettings {
  vsCodeSync: boolean;
  autoCheckUpdates: boolean;
  theme: AppTheme;
  projectIconStyle: ProjectIconStyle;
  revealShortcut: string | null;
}
