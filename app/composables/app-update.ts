export function useAppUpdate() {
  const appUpdateStore = useAppUpdateStore();

  const latestUpdate = computed(() => appUpdateStore.latestUpdate);
  const latestVersion = computed(() => latestUpdate.value?.version);
  const updateIsChecked = computed(() => appUpdateStore.updateIsChecked);

  async function checkUpdates() {
    useTauriLogInfo(`Check latest app version`);

    try {
      appUpdateStore.latestUpdate = await useTauriUpdaterCheck();

      if (appUpdateStore.latestUpdate) {
        useTauriLogInfo(`Found update ${appUpdateStore.latestUpdate.version} from ${appUpdateStore.latestUpdate.date}`);
      }

      appUpdateStore.updateIsChecked = true;
    } catch (error) {
      useTauriLogError(`Couldn't check for updates: ${error}`);
    }
  }

  return {
    latestUpdate,
    latestVersion,
    updateIsChecked,
    checkUpdates,
  };
}
