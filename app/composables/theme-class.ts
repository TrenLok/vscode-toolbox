import type { AppTheme } from '~/types/app-settings';

export function applyThemeClass(theme: AppTheme) {
  if (!import.meta.client) return;

  const root = document.documentElement;
  const classNames = root.className
    .split(/\s+/)
    .filter((className) => className && !className.startsWith('page_theme_'));

  classNames.push(`page_theme_${theme}`);
  root.className = classNames.join(' ');
}
