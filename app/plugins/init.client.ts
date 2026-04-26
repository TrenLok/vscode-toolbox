export default defineNuxtPlugin(async () => {
  const appSettings = useAppSettings();
  await appSettings.loadFromDb();

  const projectManager = useProjectManager();
  await projectManager.loadFromDb();
  if (appSettings.vsCodeSync.value) {
    projectManager.syncVSCodeRecent();
  }
  projectManager.checkBadFolders();

  const autostart = useAutostart();
  autostart.updateState();

  const hiddenFolders = useHiddenFolders();
  await hiddenFolders.loadFromDb();

  const appUpdate = useAppUpdate();

  if (appSettings.autoCheckUpdates.value) {
    appUpdate.checkUpdates();
  }

  const availableThemes = await useAvailableThemes();

  if (!availableThemes.includes(appSettings.theme.value)) {
    await appSettings.switchTheme('default');
  }

  watchEffect(() => {
    applyThemeClass(appSettings.theme.value);
  });

  globalThis.setInterval(async () => {
    await projectManager.checkBadFolders();
  }, 10_000); // 10 seconds
});
